use crate::camera::Camera;
use crate::color::Color;
use crate::frame_buffer::{FrameBuffer, Pixel};
use crate::hit::Hit;
use crate::lights::light::Light;
use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use rayon::prelude::*;
use std::fmt::Debug;

// Scene object
#[derive(Debug)]
pub struct Scene {
    lights: Vec<Box<dyn Light + Sync>>,
    primitives: Vec<PrimitiveWrapper>,
    materials: Vec<Box<dyn Material + Sync>>,
    objects: Vec<Box<dyn Object + Sync>>,
    camera: Camera,
    bvh: BVH,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<dyn Object + Sync>>,
        lights: Vec<Box<dyn Light + Sync>>,
        materials: Vec<Box<dyn Material + Sync>>,
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
            .map(|(i, o)| o.primitives(i))
            .flatten()
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

        // Build photon maps
        // TODO: Normal photon map
        // TODO: Caustics

        Scene {
            lights,
            camera,
            materials,
            primitives,
            objects,
            bvh,
        }
    }

    pub fn get_lights(&self) -> &Vec<Box<dyn Light + Sync>> {
        &self.lights
    }
}

impl<'a> Scene {
    // Calculates the color for a ray in the scene
    pub fn calc_ray(
        &self,
        ray: Ray,
        reflection_power: Color,
        reflection_depth: usize,
    ) -> (Color, f32) {
        let mut intersections = self
            .intersection(ray)
            .filter(|s| s.get_dir() && s.get_distance() > 0.)
            .collect::<Vec<Hit>>();

        if let Some(v) = intersections.first() {
            (
                self.materials[self.objects[v.get_object_index()].get_material(v)].compute(
                    ray,
                    &v,
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

        let pixels: Vec<(usize, usize, Pixel)> = (0..height)
            .collect::<Vec<usize>>()
            .par_iter()
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

        pixels.iter().for_each(|(x, y, p)| {
            fb.plot_pixel(*x, *y, p.red, p.green, p.blue);
            fb.plot_depth(*x, *y, p.depth);
        });

        fb
    }

    // Finds all the hits for a ray in the scene
    pub fn intersection(&'a self, ray: Ray) -> impl Iterator<Item = Hit> + 'a {
        let mut hits = self
            .bvh
            .traverse(&ray.bvh_ray(), &self.primitives)
            .into_iter()
            .flat_map(move |o| {
                let hits = o.primitive.intersection(&ray);
                hits
            })
            .collect::<Vec<Hit>>();

        hits.sort_by(|l, r| l.get_distance().partial_cmp(&r.get_distance()).unwrap());

        for (i, o) in self.objects.iter().enumerate() {
            hits = o.filter_hits(hits, i);
        }

        hits.into_iter()
    }
}

// Wrapper necessary for polymorphic traits
#[derive(Debug)]
struct PrimitiveWrapper {
    primitive: Box<dyn Primitive + Sync + Send>,
}

impl<'a> Bounded for PrimitiveWrapper {
    fn aabb(&self) -> AABB {
        self.primitive.aabb()
    }
}
impl<'a> BHShape for PrimitiveWrapper {
    fn set_bh_node_index(&mut self, n: usize) {
        self.primitive.set_bh_node_index(n)
    }

    fn bh_node_index(&self) -> usize {
        self.primitive.bh_node_index()
    }
}
