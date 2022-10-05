use crate::camera::Camera;
use crate::color::Color;
use crate::frame_buffer::{FrameBuffer, Pixel};
use crate::poly_mesh::PolyMesh;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Lights>,
    pub camera: Camera,
}

#[derive(Debug)]
pub enum Object {
    Sphere {
        rad: f32,
        center: Vertex,
        color: Color,
    },
    Poly {
        poly: PolyMesh,
        color: Color,
    },
    Plane {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
    },
}

#[derive(Debug)]
pub enum Lights {
    Directional {
        direction: Vector,
        position: Vertex,
        color: Color,
    },
}

impl Ray {
    pub fn intersection(&self, obj: &Object) -> Option<f32> {
        match obj {
            Object::Sphere { .. } => None,
            Object::Poly { poly, color } => poly.intersections(self),
            Object::Plane { .. } => None,
        }
    }
}

impl Scene {
    pub fn render(&self, width: usize, height: usize) -> FrameBuffer {
        let mut fb = FrameBuffer::new(width, height);
        for y in 0..height {
            println!("y: {}", y);
            for x in 0..width {
                let ray = self.camera.ray(
                    (2. * x as f32 - width as f32) / width as f32,
                    (2. * -(y as f32) + height as f32) / width as f32,
                );

                if let Some(v) = self
                    .objects
                    .iter()
                    .filter_map(|o| ray.intersection(o))
                    .next()
                {
                    println!("{}", v);
                    fb.plot_pixel(x, y, 1. / (v - 8.0), 0., 0.)
                }
            }
        }

        fb
    }
}
