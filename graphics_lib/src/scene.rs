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

#[derive(Debug)]
pub struct Scene {
    lights: Vec<Box<dyn Light + Sync>>,
    primitives: Vec<PrimitiveWrapper>,
    materials: Vec<Box<dyn Material + Sync>>,
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
        let mut primitives: Vec<PrimitiveWrapper> = objects
            .into_iter()
            .map(|o| o.primitives())
            .flatten()
            .map(|p| PrimitiveWrapper { primitive: p })
            .collect();

        let bvh = BVH::build(&mut primitives);

        let materials = materials
            .into_iter()
            .enumerate()
            .map(|(i, mut m)| {
                m.update_mat_index(i);
                m
            })
            .collect();

        Scene {
            lights,
            camera,
            materials,
            primitives,
            bvh,
        }
    }

    pub fn get_lights(&self) -> &Vec<Box<dyn Light + Sync>> {
        &self.lights
    }
}

impl<'a> Scene {
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

        intersections.sort_by(|l, r| l.get_distance().partial_cmp(&r.get_distance()).unwrap());

        if let Some(v) = intersections.first() {
            (
                self.materials[v.get_object().get_material()].compute(
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

    pub fn intersection(&'a self, ray: Ray) -> impl Iterator<Item = Hit<'a>> + 'a {
        self.bvh
            .traverse(&ray.bvh_ray(), &self.primitives)
            .into_iter()
            .flat_map(move |o| o.primitive.intersection(&ray))
    }
}

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
