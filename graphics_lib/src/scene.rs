use crate::camera::Camera;
use crate::color::Color;
use crate::constants::{EPSILON, NUMBER_PHOTONS_PER_LIGHT};
use crate::frame_buffer::{FrameBuffer, Pixel};
use crate::hit::Hit;
use crate::lights::light::Light;
use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::photon::Photon;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use kd_tree::KdTree;
use rayon::prelude::*;
use std::fmt::Debug;

// Scene object
#[derive(Debug)]
pub struct Scene {
    lights: Vec<Box<dyn Light + Sync + Send>>,
    primitives: Vec<PrimitiveWrapper>,
    materials: Vec<Box<dyn Material + Sync + Send>>,
    objects: Vec<Box<dyn Object + Sync + Send>>,
    camera: Camera,
    bvh: BVH,
    photon_map: Option<KdTree<Photon>>,
    caustic_map: Option<KdTree<Photon>>,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<dyn Object + Sync + Send>>,
        lights: Vec<Box<dyn Light + Sync + Send>>,
        materials: Vec<Box<dyn Material + Sync + Send>>,
        camera: Camera,
    ) -> Scene {
        // Store objects and populate each objects CSG tree indices
        let mut objects = objects;
        for o in objects.iter_mut() {
            o.set_csg_index(1);
        }

        // Get the primitive from each object (with the objects index)
        let mut primitives: Vec<PrimitiveWrapper> = objects
            .iter()
            .enumerate()
            .flat_map(|(i, o)| o.primitives(i))
            .map(|p| PrimitiveWrapper { primitive: p })
            .collect();

        // Build the bounding view hierarchy for the scene
        let bvh = BVH::build(&mut primitives);

        // Tell each material its index (necessary for refraction logic)
        let materials = materials
            .into_iter()
            .enumerate()
            .map(|(i, mut m)| {
                m.update_mat_index(i);
                m
            })
            .collect();

        let mut scene = Scene {
            lights,
            camera,
            materials,
            primitives,
            objects,
            bvh,
            // These will be instantly populated
            // Only not populated as it it useful to have the scene initialised
            //   before populating
            photon_map: None,
            caustic_map: None,
        };

        // Build photon maps
        scene.photon_map = Some(scene.photon_map());

        // Return scene
        scene
    }

    pub fn get_lights(&self) -> &Vec<Box<dyn Light + Sync + Send>> {
        &self.lights
    }

    // Calculates the color for a ray in the scene
    pub fn calc_ray(
        &self,
        ray: Ray,
        reflection_power: Color,
        reflection_depth: usize,
    ) -> (Color, f32) {
        let intersections = self
            .intersection(ray)
            .filter(|s| s.get_dir() && s.get_distance() > 0.)
            .collect::<Vec<Hit>>();

        if let Some(v) = intersections.first() {
            (
                self.materials[self.objects[v.get_object_index()].get_material(v)].compute(
                    ray,
                    v,
                    Color::new(1., 1., 1.),
                    self,
                    reflection_depth,
                    reflection_power,
                ),
                v.get_distance().min(100.),
            )
        } else {
            (Color::new(0.0, 0.0, 0.0), 0.0)
        }
    }

    // Renders an image in the scene
    pub fn render(&self, width: usize, height: usize) -> FrameBuffer {
        let mut fb = FrameBuffer::new(width, height);

        // Builds a Vector of pixels in parallel
        let pixels: Vec<(usize, usize, Pixel)> = (0..height)
            .collect::<Vec<usize>>()
            .par_iter() // Parallel iterator, courtesy of Rayon
            .map(move |y| {
                (0..width)
                    .map(|x| {
                        let ray = self.camera.ray(
                            (2. * (x as f32) - width as f32) / width as f32,
                            (2. * -(*y as f32) + height as f32) / width as f32,
                        );

                        let res = self.calc_ray(ray, Color::new(1., 1., 1.), 0);

                        (x, *y, Pixel::from_color(res.0, res.1))
                    })
                    .collect::<Vec<(usize, usize, Pixel)>>()
            })
            .flatten()
            .collect();
        // Doesnt populate the FrameBuffer in parallel to avoid memory safety

        // From the pixels
        pixels.iter().for_each(|(x, y, p)| {
            fb.plot_pixel(*x, *y, p.red, p.green, p.blue);
            fb.plot_depth(*x, *y, p.depth);
        });

        fb
    }

    // Finds all the hits for a ray in the scene
    pub fn intersection(&self, ray: Ray) -> impl Iterator<Item = Hit> {
        let mut hits = self
            .bvh
            .traverse(&ray.bvh_ray(), &self.primitives)
            .into_iter()
            .flat_map(move |o| o.primitive.intersection(&ray))
            .collect::<Vec<Hit>>();
        // NOTE: at this point this is every intersection with every primitive
        // Some of these primitives will be CSG's so some of these hits may
        //   later be removed or have direction/normal modified

        // Sorts the hits by position along ray (negative values included)
        hits.sort_by(|l, r| l.get_distance().partial_cmp(&r.get_distance()).unwrap());

        // Allows objects to remove or modify hits
        //  This is only used by CSG's to remove and modify hits
        //  Note: MUST be sorted before handed to CSG for internal logic
        for (i, o) in self.objects.iter().enumerate() {
            hits = o.filter_hits(hits, i);
        }

        hits.into_iter()
    }

    fn photon_map(&self) -> KdTree<Photon> {
        // TODO: Normal photon map
        // TODO: Caustics
        let photons: Vec<Photon> = self
            .lights
            .iter() // For each light
            .enumerate()
            .flat_map(|(i, light)| {
                (0..NUMBER_PHOTONS_PER_LIGHT) // Repeat this many times
                    .into_par_iter() // In parallel
                    .flat_map::<_, Vec<Photon>>(move |_| {
                        let ray = light.generate_photon_dir(); // Generate random ray from light
                        self.calculate_photon_ray(ray, i) // Get the photons from that ray
                    })
                    .collect::<Vec<Photon>>()
            })
            .collect();
        KdTree::build_by_ordered_float(photons) // Collate photons into tree
    }

    fn calculate_photon_ray(&self, ray: Ray, light_index: usize) -> Vec<Photon> {
        // list of intersections sorted and filtered
        let mut hits = self
            .intersection(ray)
            .into_iter()
            .filter(|h| h.get_distance() > EPSILON);

        let Some(direct_hit) = hits.next() else {
            return vec![];
        };

        // Ask material to compute the photons for the direct and indirect photons
        let mut res = self.materials
            [self.objects[direct_hit.get_object_index()].get_material(&direct_hit)]
        .compute_photon(ray, &direct_hit, self, 0, Color::new_grey(1.));

        // Add in shadow photons for all subsequent hits
        res.append(
            &mut hits
                .map(|h| Photon::new_shadow(*h.pos(), light_index))
                .collect::<Vec<Photon>>(),
        );
        res
    }
}

// Wrapper necessary for polymorphic traits
#[derive(Debug)]
struct PrimitiveWrapper {
    primitive: Box<dyn Primitive + Sync + Send>,
}

impl Bounded for PrimitiveWrapper {
    fn aabb(&self) -> AABB {
        self.primitive.aabb()
    }
}
impl BHShape for PrimitiveWrapper {
    fn set_bh_node_index(&mut self, n: usize) {
        self.primitive.set_bh_node_index(n)
    }

    fn bh_node_index(&self) -> usize {
        self.primitive.bh_node_index()
    }
}
