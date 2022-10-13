use crate::camera::Camera;
use crate::frame_buffer::{FrameBuffer, Pixel};
use crate::hit::Hit;
use crate::object::Object;
use rayon::prelude::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object + Sync>>,
    pub lights: Vec<Box<dyn Lights + Sync>>,
    pub camera: Camera,
}

pub trait Lights: Debug {}

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
                                    .map(|s| (s, o, self.camera.position.distance(s.pos)))
                            })
                            .collect::<Vec<(Hit, &Box<dyn Object + Sync>, f32)>>();

                        intersections.sort_by(|l, r| l.2.partial_cmp(&r.2).unwrap());

                        if let Some(v) = intersections.first() {
                            (
                                x,
                                *y,
                                Pixel::from_color(v.1.get_material().compute_once(&ray, &v.0), 0.),
                            )
                        } else {
                            (x, *y, Pixel::from_colors(0.0, 0.0, 0.0, 0.0))
                        }
                    })
                    .collect::<Vec<(usize, usize, Pixel)>>()
            })
            .flatten()
            .collect();

        pixels
            .iter()
            .for_each(|(x, y, p)| fb.plot_pixel(*x, *y, p.red, p.green, p.blue));

        fb
    }
}
