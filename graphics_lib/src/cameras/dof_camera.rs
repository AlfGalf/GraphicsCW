use crate::cameras::camera::Camera;
use crate::ray::Ray;
use glam::Vec3;
use rand::Rng;
use std::f32::consts::PI;

// Camera with depth of field effect
#[derive(Debug)]
pub struct DoFCamera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    right: Vec3,
    zoom: f32,
    focal_length: f32,
    lens_width: f32,
    num_rays: usize,
}

impl DoFCamera {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        up: Vec3,
        zoom: f32,
        num_rays: usize,
        focal_length: f32,
        lens_width: f32,
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
    fn rays(&self, x: f32, y: f32) -> Vec<Ray> {
        let mut rng = rand::thread_rng();

        // Direction if this was a camera without DOF effect
        let normal_dir = ((self.direction * self.zoom)
            + (self.up * (y as f32 / 2.))
            + (self.right * (x as f32 / 2.)))
            .normalize();

        // Work out focal point from normal camera ray
        let focal_point = self.position + normal_dir * self.focal_length;

        // Generate a number of rays from within the lens to the focal point
        (0..self.num_rays)
            .map(|_| {
                // from https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
                let r: f32 = rng.gen_range::<f32, _>(0. ..1.).sqrt() * self.lens_width;
                let theta: f32 = rng.gen_range(0. ..2. * PI);

                let point =
                    self.position + (r * theta.cos()) * self.up + (r * theta.sin()) * self.right;

                Ray::new(point, focal_point - point)
            })
            .collect()
    }
}
