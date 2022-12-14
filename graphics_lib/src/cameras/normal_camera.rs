use crate::cameras::camera::Camera;
use crate::ray::Ray;
use glam::DVec3;

// A camera that generates one ray per pixel
#[derive(Debug)]
pub struct NormalCamera {
    position: DVec3,
    direction: DVec3,
    up: DVec3,
    right: DVec3,
    zoom: f64,
}

impl NormalCamera {
    pub fn new(position: DVec3, direction: DVec3, up: DVec3, zoom: f64) -> Self {
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
    fn rays(&self, x: f64, y: f64) -> Vec<Ray> {
        vec![Ray::new(
            self.position,
            (self.direction * self.zoom)
                + (self.up * (y as f64 / 2.))
                + (self.right * (x as f64 / 2.)),
        )]
    }
}
