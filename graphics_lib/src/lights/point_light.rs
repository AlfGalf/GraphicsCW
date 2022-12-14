use crate::color::Color;
use crate::fibonacci_spiral::fibonacci_spiral_random;
use crate::lights::light::Light;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::DVec3;
use rand::Rng;

const EPSILON: f64 = 0.01;

#[derive(Debug)]
pub struct PointLight {
    position: DVec3,
    color: Color,
}

impl PointLight {
    pub fn new(position: DVec3, color: Color) -> Self {
        Self { position, color }
    }
}

impl Light for PointLight {
    fn get_intensity(&self, point: DVec3, scene: &Scene, light_index: usize) -> Color {
        let distance = point.distance(self.position);
        let ray = Ray::new(self.position, (point - self.position).normalize());

        // If there are shadow photons and no direct photons, then it is dark,
        // And vice versa
        // Otherwise, send a shadow ray
        let photons = scene.get_photons(point, 0.1);
        let num_direct = photons
            .iter()
            .filter(|p| p.is_direct() && p.get_light_index() == light_index)
            .count();
        let num_shadow = photons
            .iter()
            .filter(|p| p.is_shadow() && p.get_light_index() == light_index)
            .count();

        if num_shadow > 2 && num_direct == 0
            || (!(num_direct > 2 && num_shadow == 0)
                && scene.intersection(ray).any(|r| {
                    r.get_dir() && r.get_distance() > 0. && r.get_distance() < distance - EPSILON
                }))
        {
            Color::new_black()
        } else {
            self.color * (1. / ((1. + distance / 10.) * (1. + distance / 10.)))
        }
    }

    fn get_direction(&self, point: DVec3) -> DVec3 {
        (point - self.position).normalize()
    }

    fn generate_photon_dir(&self) -> Ray {
        Ray::new(self.position, fibonacci_spiral_random())
    }

    fn generate_caustic_dir(&self, bounds: (DVec3, DVec3)) -> Ray {
        if !(bounds.0.is_finite() && bounds.1.is_finite()) {
            panic!("Non finite bounds, you may need to wrap an object in a CSG.");
        }

        let mut rng = rand::thread_rng();
        let x_rand: f64 = rng.gen_range((bounds.0.x)..bounds.1.x);
        let y_rand: f64 = rng.gen_range((bounds.0.y)..bounds.1.y);
        let z_rand: f64 = rng.gen_range((bounds.0.z)..bounds.1.z);
        Ray::new(
            self.position,
            DVec3::new(x_rand, y_rand, z_rand) - self.position,
        )
    }

    fn get_color(&self) -> Color {
        self.color
    }
}
