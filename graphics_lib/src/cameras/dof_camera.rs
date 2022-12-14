use crate::cameras::camera::Camera;
use crate::ray::Ray;
use glam::DVec3;
use rand::Rng;
use std::f64::consts::PI;

// Camera with depth of field effect
#[derive(Debug)]
pub struct DoFCamera {
    position: DVec3,
    direction: DVec3,
    up: DVec3,
    right: DVec3,
    zoom: f64,
    focal_length: f64,
    lens_width: f64,
    num_rays: usize,
}

impl DoFCamera {
    pub fn new(
        position: DVec3,
        direction: DVec3,
        up: DVec3,
        zoom: f64,
        num_rays: usize,
        focal_length: f64,
        lens_width: f64,
    ) -> Self {
        let right = up.cross(direction).normalize();
        let up = direction.cross(right).normalize();
        Self {
            position,
            direction,
            up,
            right,
            zoom,
            focal_length,
            lens_width,
            num_rays,
        }
    }
}

impl Camera for DoFCamera {
    // x should vary -1 -> 1
    // y should vary -a -> a
    fn rays(&self, x: f64, y: f64) -> Vec<Ray> {
        let mut rng = rand::thread_rng();

        // Direction if this was a camera without DOF effect
        let normal_dir = ((self.direction * self.zoom)
            + (self.up * (y as f64 / 2.))
            + (self.right * (x as f64 / 2.)))
            .normalize();

        // Work out focal point from normal camera ray
        let focal_point = self.position + normal_dir * self.focal_length;

        // Generate a number of rays from within the lens to the focal point
        (0..self.num_rays)
            .map(|_| {
                // from https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
                let r: f64 = rng.gen_range::<f64, _>(0. ..1.).sqrt() * self.lens_width;
                let theta: f64 = rng.gen_range(0. ..2. * PI);

                let point =
                    self.position + (r * theta.cos()) * self.up + (r * theta.sin()) * self.right;

                Ray::new(point, focal_point - point)
            })
            .collect()
    }
}
