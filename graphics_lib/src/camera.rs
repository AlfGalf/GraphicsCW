use crate::ray::Ray;
use glam::Vec3;

#[derive(Debug)]
pub struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    focal_length: f32,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, up: Vec3, focal_length: f32) -> Camera {
        Camera {
            position,
            direction,
            up,
            focal_length,
        }
    }

    // x should vary -1 -> 1
    // y should vary -a -> a
    pub fn ray(&self, x: f32, y: f32) -> Ray {
        let right = self.up.cross(self.direction).normalize();
        let up = self.direction.cross(right);

        Ray::new(
            self.position,
            (self.direction * self.focal_length)
                + (up * (y as f32 / 2.))
                + (right * (x as f32 / 2.)),
        )
    }
}
