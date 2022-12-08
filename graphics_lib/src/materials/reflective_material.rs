use crate::color::Color;
use crate::constants::{EPSILON, MAX_RECURSE_DEPTH, MIN_RECURSE_COEFFICIENT};
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;

// This material is a perfectly reflective material
#[derive(Clone, Debug)]
pub struct ReflectiveMaterial {
    mat_index: usize,
}

impl ReflectiveMaterial {
    pub(crate) fn new() -> ReflectiveMaterial {
        ReflectiveMaterial { mat_index: 0 }
    }
}

impl Material for ReflectiveMaterial {
    fn compute(
        &self,
        view_ray: Ray,
        hit: &Hit,
        _: Color,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Color {
        if recurse_depth < MAX_RECURSE_DEPTH && recurse_power.min_val() > MIN_RECURSE_COEFFICIENT {
            let reflection_dir: Vec3 = view_ray.direction()
                - 2. * (view_ray.direction().dot(*hit.normal())) * *hit.normal();
            let reflection_dir = reflection_dir.normalize();

            let reflection_ray = Ray::new(*hit.pos() + reflection_dir * EPSILON, reflection_dir);
            let (col, _) = scene.calc_ray(reflection_ray, recurse_power, recurse_depth + 1);

            col.piecewise_mul(&recurse_power)
        } else {
            Color::new_black()
        }
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
        let reflection_dir: Vec3 =
            view_ray.direction() - 2. * (view_ray.direction().dot(*hit.normal())) * *hit.normal();
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
        // Does not retransmit caustics
        // TODO: Could change this?
        None
    }
}
