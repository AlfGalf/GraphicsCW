use crate::camera::Camera;
use crate::color::Color;
use crate::frame_buffer::{FrameBuffer, Pixel};
use crate::hit::Hit;
use crate::poly_mesh::PolyMesh;
use crate::ray::Ray;
use glam::Vec3;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Lights>,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Object {
    obj: ObjectEnum,
    color: Color,
}

impl Object {
    pub fn new_poly(mesh: PolyMesh, color: Color) -> Self {
        Self {
            obj: ObjectEnum::Poly { poly: mesh },
            color,
        }
    }

    pub fn new_sphere(center: Vec3, rad: f32, color: Color) -> Self {
        Self {
            obj: ObjectEnum::Sphere { rad, center },
            color,
        }
    }
}

#[derive(Debug)]
pub enum ObjectEnum {
    Sphere { rad: f32, center: Vec3 },
    Poly { poly: PolyMesh },
    Plane { a: f32, b: f32, c: f32, d: f32 },
}

#[derive(Debug)]
pub enum Lights {
    Directional {
        direction: Vec3,
        position: Vec3,
        color: Color,
    },
}

impl Ray {
    pub fn intersection(&self, object: &Object) -> Option<Hit> {
        match &object.obj {
            ObjectEnum::Sphere { center, rad } => {
                // (D.D)*t2+(D.P)*2t+(P.P–R2) =0
                let relative_position: Vec3 = self.position - *center;
                let b = 2. * self.direction.dot(relative_position);
                let c = relative_position.dot(relative_position) - rad * rad;

                let discriminant: f32 = b * b - 4. * c;
                if discriminant <= 0. {
                    None
                } else {
                    let sqrt = discriminant.sqrt();
                    let t0 = (sqrt - b) / 2.;
                    let t1 = (-sqrt - b) / 2.;
                    let t = t0.min(t1);
                    if t <= 0. {
                        None
                    } else {
                        let pos = self.position + self.direction * t;
                        Some(Hit::new(pos, (pos - *center).normalize()))
                    }
                }
            }
            ObjectEnum::Poly { poly, .. } => poly.intersections(self),
            ObjectEnum::Plane { .. } => None,
        }
    }
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
                            .filter_map(|o| (ray.intersection(o).map(|s| (s, o))))
                            .collect::<Vec<(Hit, &Object)>>();
                        intersections.sort_by(|l, r| {
                            self.camera
                                .position
                                .distance(l.0.pos)
                                .partial_cmp(&self.camera.position.distance(r.0.pos))
                                .unwrap()
                        });

                        if let Some(v) = intersections.first() {
                            (
                                x,
                                *y,
                                Pixel::from_colors(
                                    (v.0.normal.x + 1.0) * 0.5,
                                    (v.0.normal.y + 1.0) * 0.5,
                                    (v.0.normal.z + 1.0) * 0.5,
                                    0.,
                                ),
                                // Pixel::from_colors(
                                //     v.1.color.red,
                                //     v.1.color.green,
                                //     v.1.color.blue,
                                //     0.,
                                // ),
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
