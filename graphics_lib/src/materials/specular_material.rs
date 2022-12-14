use crate::color::Color;
use crate::fibonacci_spiral::fibonacci_spiral_random;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::DVec3;
use std::fmt::Debug;

// This material gives specular highlights
//      As defined in the Phong lighting model
#[derive(Debug, Clone)]
pub struct SpecularMaterial {
    power: i32,
}

impl SpecularMaterial {
    pub fn new(power: i32) -> SpecularMaterial {
        SpecularMaterial { power }
    }
}

impl Material for SpecularMaterial {
    fn compute<'a>(
        &self,
        view_ray: Ray,
        hit: &Hit,
        _: Color,
        scene: &Scene,
        _: usize,
        _: Color,
    ) -> Color {
        // finds the reflection direction of the ray viewing this point
        let reflection_dir: DVec3 =
            view_ray.direction() - 2. * (view_ray.direction().dot(*hit.normal())) * *hit.normal();

        let reflection_dir = reflection_dir.normalize();

        // Calculate for each light then sum the results
        scene
            .get_lights()
            .iter()
            .enumerate()
            .fold(Color::new_black(), |c, (i, light)| {
                let intensity = light.get_intensity(*hit.pos(), scene, i);
                let dir = light.get_direction(*hit.pos());

                // Calculates the specular coefficient of this light
                // Phong lighing model
                let specular = reflection_dir.dot(-dir).powi(self.power as i32).max(0.);

                c + intensity * specular
            })
    }

    fn compute_photon(
        &self,
        view_ray: Ray,
        hit: &Hit,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
        light_index: usize,
    ) -> Vec<Photon> {
        // Photon generation in the opposite direction with a little bit of randomness
        let reflection_dir: DVec3 =
            view_ray.direction() - 2. * (view_ray.direction().dot(*hit.normal())) * *hit.normal();

        let reflection_dir = reflection_dir + 0.1 * fibonacci_spiral_random();
        let reflection_dir = reflection_dir.normalize();

        scene.calculate_photon_ray(
            Ray::new(*hit.pos(), reflection_dir),
            light_index,
            recurse_depth,
            recurse_power,
        )
    }

    fn needs_caustic(&self) -> bool {
        false
    }

    fn compute_caustic_ray(
        &self,
        _view_ray: Ray,
        _hit: &Hit,
        _scene: &Scene,
        _recurse_depth: usize,
        _light_index: usize,
        _: Color,
    ) -> Option<Photon> {
        // This does not retransmit caustics
        None
    }
}
