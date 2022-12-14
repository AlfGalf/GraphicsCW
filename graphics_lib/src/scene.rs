use crate::cameras::camera::Camera;
use crate::color::Color;
use crate::constants::{
    EPSILON, MAX_PHOTON_RECURSE_DEPTH, MIN_RECURSE_COEFFICIENT, NUMBER_CAUSTICS_PER_LIGHT_PER_OBJ,
    NUMBER_PHOTONS_PER_LIGHT,
};
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
use glam::DVec3;
use kd_tree::KdTree;
use rayon::prelude::*;
use std::fmt::Debug;
use std::sync::Mutex;

// Scene object
#[derive(Debug)]
pub struct Scene {
    lights: Vec<Box<dyn Light + Sync + Send>>,
    primitives: Vec<PrimitiveWrapper>,
    materials: Vec<Box<dyn Material + Sync + Send>>,
    objects: Vec<Box<dyn Object + Sync + Send>>,
    camera: Box<dyn Camera + Sync + Send>,
    // The Bounding View Hierarchy data structure is an external crate
    // https://crates.io/crates/bvh
    bvh: BVH,
    // KdTree is an external crate used
    // https://crates.io/crates/kdtree
    photon_map: KdTree<Photon>,
    caustic_map: KdTree<Photon>,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<dyn Object + Sync + Send>>,
        lights: Vec<Box<dyn Light + Sync + Send>>,
        materials: Vec<Box<dyn Material + Sync + Send>>,
        camera: Box<dyn Camera + Sync + Send>,
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
            photon_map: KdTree::default(),
            caustic_map: KdTree::default(),
        };

        println!("-- Made scene --");

        // Build photon maps
        scene.photon_map = scene.photon_map();

        println!("-- Built photon map --");

        // Build photon maps
        scene.caustic_map = scene.caustic_map();

        println!("-- Built caustic map --");

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
    ) -> (Color, f64) {
        let intersections = self
            .intersection(ray)
            .filter(|s| s.get_dir() && s.get_distance() > 0.)
            .collect::<Vec<Hit>>();

        // Only consider the case where it hits something, otherwise return black
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

        let mut done_count: Mutex<usize> = Mutex::new(0);

        // Builds a Vector of pixels in parallel
        let pixels: Vec<(usize, usize, Pixel)> = (0..height)
            .collect::<Vec<usize>>()
            .par_iter() // Parallel iterator, courtesy of Rayon
            // https://crates.io/crates/rayon
            .map(move |y| {
                // println!("line {} done", *y);
                let res = (0..width)
                    .map(|x| {
                        let rays = self.camera.rays(
                            (2. * (x as f64) - width as f64) / width as f64,
                            (2. * -(*y as f64) + height as f64) / width as f64,
                        );

                        // For each ray the camera gives calculate, then average the result
                        let (col_acc, depth_acc) = rays.iter().fold(
                            (Color::new_black(), 0.),
                            |(col_acc, depth_acc), ray| {
                                let (col, depth) = self.calc_ray(*ray, Color::new_grey(1.), 0);

                                (col_acc + col, depth_acc + depth)
                            },
                        );

                        // Return the resulting pixel
                        (
                            x,
                            *y,
                            Pixel::from_color(
                                col_acc * (1. / rays.len() as f64),
                                depth_acc / rays.len() as f64,
                            ),
                        )
                    })
                    .collect::<Vec<(usize, usize, Pixel)>>();
                let mut data = done_count.lock().unwrap();
                *data += 1;
                println!("{}%", *data as f32 * 100. / height as f32);
                res
            })
            .flatten()
            .collect();

        println!("-- Done rendering --");
        // Doesnt populate the FrameBuffer in parallel to avoid parallel
        //  memory safety diffculties

        // From the frame buffer from the pixel results
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

    // Calculates the photon map for the scene
    fn photon_map(&self) -> KdTree<Photon> {
        let photons: Vec<Photon> = self
            .lights
            .par_iter() // For each light
            .enumerate()
            .flat_map(|(i, light)| {
                (0..NUMBER_PHOTONS_PER_LIGHT) // Repeat this many times
                    .into_par_iter() // In parallel
                    .flat_map::<_, Vec<Photon>>(move |_| {
                        let ray = light.generate_photon_dir(); // Generate random ray from light
                        self.calculate_photon_ray(ray, i, 0, light.get_color())
                        // Get the photons from that ray
                    })
                    .collect::<Vec<Photon>>()
            })
            .collect();

        KdTree::build_by_ordered_float(photons) // Collate photons into tree
    }

    // Calculates a list of photons for a ray of light in a scene
    // This can be called recursively by the materials
    pub fn calculate_photon_ray(
        &self,
        ray: Ray,
        light_index: usize,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Vec<Photon> {
        // list of intersections sorted and filtered
        let mut hits = self
            .intersection(ray)
            .into_iter()
            .filter(|h| h.get_distance() > EPSILON);

        let Some(direct_hit) = hits.find(|h| h.get_dir()) else {
            return vec![];
        };

        // Only add a direct photon if this is the first hit
        let mut res = if recurse_depth == 0 {
            vec![Photon::new_direct(
                *direct_hit.pos(),
                light_index,
                direct_hit.get_object_index(),
            )]
        } else {
            vec![]
        };

        // Ask material to compute the photons for the direct and indirect photons
        if recurse_depth < MAX_PHOTON_RECURSE_DEPTH
            && recurse_power.max_val() > MIN_RECURSE_COEFFICIENT
        {
            res.append(
                &mut self.materials
                    [self.objects[direct_hit.get_object_index()].get_material(&direct_hit)]
                .compute_photon(
                    ray,
                    &direct_hit,
                    self,
                    recurse_depth + 1,
                    recurse_power,
                    light_index,
                ),
            );
        }

        // Add in shadow photons for all subsequent hits
        if recurse_depth == 0 {
            res.append(
                &mut hits
                    .map(|h| Photon::new_shadow(*h.pos(), light_index, h.get_object_index()))
                    .collect::<Vec<Photon>>(),
            );
        }

        res
    }

    // Calculates a single caustic photon for a ray of light in the scene pointed at a specific object
    // This can be called recursively by materials
    pub fn calculate_caustic(
        &self,
        ray: &Ray,
        object_index: usize,
        light_index: usize,
        color: Color,
        recurse_depth: usize,
    ) -> Option<Photon> {
        let intersections: Vec<Hit> = self
            .intersection(*ray)
            .filter(|h| h.get_distance() > EPSILON && h.get_dir())
            .collect();

        let Some(hit) = intersections.first() else {
            return None;
            // Ray doesn't hit anything
        };

        if hit.get_object_index() != object_index {
            // Ray does not hit the caustic object first
            if recurse_depth == 0 {
                // If this is the first recurse do nothing
                None
            } else {
                // Otherwise add a caustic there and finish
                Some(Photon::new_caustic(
                    hit.pos(),
                    light_index,
                    color,
                    object_index,
                ))
            }
        } else {
            // Get the material of the object and find compute the caustics for that material
            self.materials[self.objects[object_index].get_material(hit)].compute_caustic_ray(
                *ray,
                hit,
                self,
                recurse_depth + 1,
                light_index,
                color,
            )
        }
    }

    fn caustic_map(&self) -> KdTree<Photon> {
        // Builds the caustic KdTree
        let caustics: Vec<Photon> = self
            .objects
            .par_iter()
            .enumerate()
            // For each object
            .flat_map(|(obj_index, obj)| {
                // If the object has a material that needs caustics
                if obj.needs_caustic(self) {
                    // find the bounds to generate the caustic within
                    let caustic_box = obj.get_caustic_bounds();
                    self.lights
                        .iter()
                        .enumerate()
                        .flat_map(|(light_index, light)| {
                            (0..NUMBER_CAUSTICS_PER_LIGHT_PER_OBJ)
                                .filter_map(|_| {
                                    let ray = light.generate_caustic_dir(caustic_box);
                                    self.calculate_caustic(
                                        &ray,
                                        obj_index,
                                        light_index,
                                        light.get_color(),
                                        0,
                                    )
                                })
                                .collect::<Vec<Photon>>()
                        })
                        .collect()
                } else {
                    vec![]
                }
            })
            .collect();
        KdTree::build_by_ordered_float(caustics)
    }

    // Finds all the photons within a radius from the photon map
    pub fn get_photons(&self, pos: DVec3, rad: f64) -> Vec<Photon> {
        if self.photon_map.is_empty() {
            vec![]
        } else {
            self.photon_map
                .within_radius(&pos.to_array(), rad)
                .into_iter()
                .cloned()
                .collect()
        }
    }
    // Finds all the caustics within a radius
    pub fn get_caustics(&self, pos: DVec3, rad: f64) -> Vec<Photon> {
        if self.caustic_map.is_empty() {
            vec![]
        } else {
            self.caustic_map
                .within_radius(&pos.to_array(), rad)
                .into_iter()
                .cloned()
                .collect()
        }
    }

    pub fn material_needs_caustic(&self, mat: usize) -> bool {
        self.materials[mat].needs_caustic()
    }
}

// Wrapper necessary for polymorphic traits for primitives
// Necessary to convince the compiler a vector of Boxes of primitives implements
//      traits necessary for BVH
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
