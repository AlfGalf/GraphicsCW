use crate::color::Color;
use crate::constants::{EPSILON, MAX_RECURSE_DEPTH, MIN_RECURSE_COEFFICIENT};
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct ReflectiveMaterial {}

impl ReflectiveMaterial {
    pub(crate) fn new() -> ReflectiveMaterial {
        ReflectiveMaterial {}
    }
}

impl Material for ReflectiveMaterial {
    fn compute(
        &self,
        view_ray: &Ray,
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
            let (col, _) = scene.calc_ray(&reflection_ray, recurse_power, recurse_depth + 1);

            col.scale(&recurse_power)
        } else {
            Color::new_black()
        }
    }

    fn clone_dyn(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
