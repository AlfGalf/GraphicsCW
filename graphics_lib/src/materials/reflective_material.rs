use crate::color::Color;
use crate::constants::{EPSILON, MAX_RECURSE_DEPTH, MIN_RECURSE_COEFFICIENT};
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;

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

            col.scale(&recurse_power)
        } else {
            Color::new_black()
        }
    }

    fn update_mat_index(&mut self, i: usize) {
        self.mat_index = i
    }

    fn get_mat_index(&self) -> usize {
        self.mat_index
    }
}
