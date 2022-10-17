use crate::camera::Camera;
use crate::color::Color;
use crate::frame_buffer::{FrameBuffer, Pixel};
use crate::hit::Hit;
use crate::lights::light::Light;
use crate::objects::object::Object;
use crate::ray::Ray;
use glam::Vec3;
use rayon::prelude::*;
use std::fmt::Debug;
use std::iter::FilterMap;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object + Sync>>,
    pub lights: Vec<Box<dyn Light + Sync>>,
    pub camera: Camera,
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
                            .intersection(&ray)
                            .filter(|s| s.get_distance() > 0.)
                            .collect::<Vec<Hit>>();

                        intersections.sort_by(|l, r| {
                            l.get_distance().partial_cmp(&r.get_distance()).unwrap()
                        });

                        if let Some(v) = intersections.first() {
                            (
                                x,
                                *y,
                                Pixel::from_color(
                                    v.get_object().get_material().compute_once(
                                        &ray,
                                        &v,
                                        Color::new(1., 1., 1.),
                                    ) + self
                                        .lights
                                        .iter()
                                        .map(|l| {
                                            v.get_object().get_material().compute_per_light(
                                                &ray,
                                                &v,
                                                &l.get_direction(&v.pos),
                                                l.get_intensity(&v.pos, self),
                                            )
                                        })
                                        .sum(),
                                    v.get_distance().min(100.),
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

    pub fn intersection<'a>(&'a self, ray: &'a Ray) -> impl Iterator<Item = Hit> + '_ {
        self.objects.iter().filter_map(|o| o.intersection(ray))
    }
}
