use crate::cameras::camera::Camera;
use crate::ray::Ray;
use glam::Vec3;

// A camera that generates one ray per pixel
#[derive(Debug)]
pub struct NormalCamera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    right: Vec3,
    zoom: f32,
}

impl NormalCamera {
    pub fn new(position: Vec3, direction: Vec3, up: Vec3, zoom: f32) -> Self {
        let right = up.cross(direction).normalize();
        let up = direction.cross(right).normalize();
        Self {
            position,
            direction,
            up,
            right,
            zoom,
        }
    }
}

impl Camera for NormalCamera {
    // x should vary -1 -> 1
    // y should vary -a -> a
    // a varies with the aspect ratio of the image
    fn rays(&self, x: f32, y: f32) -> Vec<Ray> {
        vec![Ray::new(
            self.position,
            (self.direction * self.zoom)
                + (self.up * (y as f32 / 2.))
                + (self.right * (x as f32 / 2.)),
        )]
    }
}
