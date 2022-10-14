use crate::camera::Camera;
use crate::color::Color;
use crate::frame_buffer::{FrameBuffer, Pixel};
use crate::hit::Hit;
use crate::objects::object::Object;
use glam::Vec3;
use rayon::prelude::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object + Sync>>,
    pub lights: Vec<Box<dyn Light + Sync>>,
    pub camera: Camera,
}

pub trait Light: Debug {
    fn get_intensity(&self, point: &Vec3, scene: &Scene) -> Color;
    fn get_direction(&self, point: &Vec3) -> Vec3;
}

impl Scene {
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

                        let mut intersections = self
                            .objects
                            .iter()
                            .filter_map(|o| {
                                o.intersection(&ray)
                                    .filter(|s| s.get_distance() > 0.)
                                    .map(|s| (s, o, self.camera.position.distance(s.pos)))
                            })
                            .collect::<Vec<(Hit, &Box<dyn Object + Sync>, f32)>>();

                        intersections.sort_by(|l, r| l.2.partial_cmp(&r.2).unwrap());

                        if let Some(v) = intersections.first() {
                            (
                                x,
                                *y,
                                Pixel::from_color(
                                    v.1.get_material().compute_once(
                                        &ray,
                                        &v.0,
                                        Color::new(1., 1., 1.),
                                    ) + self
                                        .lights
                                        .iter()
                                        .map(|l| {
                                            v.1.get_material().compute_per_light(
                                                &ray,
                                                &v.0,
                                                &l.get_direction(&v.0.pos),
                                                l.get_intensity(&v.0.pos, self),
                                            )
                                        })
                                        .sum(),
                                    v.0.get_distance().min(100.),
                                ),
                            )
                        } else {
                            (x, *y, Pixel::from_colors(0.0, 0.0, 0.0, 0.0))
                        }
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
}
