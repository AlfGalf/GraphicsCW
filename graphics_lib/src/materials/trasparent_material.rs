use crate::color::Color;
use crate::constants::{EPSILON, MIN_RECURSE_COEFFICIENT};
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct TransparentMaterial {
    refractive_index: f32,
}

impl TransparentMaterial {
    pub fn new(refractive_index: f32) -> Self {
        TransparentMaterial { refractive_index }
    }
}

impl Material for TransparentMaterial {
    fn compute(
        &self,
        view_ray: &Ray,
        hit: &Hit,
        ambient: Color,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Color {
        let normal = hit.normal();

        let cos_t_i = (*normal).dot(-view_ray.direction());
        let test_part = 1. - (1. / self.refractive_index.powi(2)) * (1. - cos_t_i.powi(2));
        if test_part < 0. {
            return Color::new_black();
        }
        let cos_t_t = test_part.sqrt();

        let r_par = (self.refractive_index * cos_t_i - cos_t_t)
            / (self.refractive_index * cos_t_i + cos_t_t);
        let r_per = (cos_t_i - self.refractive_index * cos_t_t)
            / (cos_t_i + self.refractive_index * cos_t_t);

        let k_r = (r_par.powi(2) + r_per.powi(2)) / 2.;
        let k_r = k_r.min(1.).max(0.);
        let k_t = 1. - k_r;

        let refl_part = if k_r > MIN_RECURSE_COEFFICIENT {
            let reflection_dir = view_ray.direction()
                - 2. * (view_ray.direction().dot(*hit.normal())) * *hit.normal();
            let reflection_dir = reflection_dir.normalize();
            let reflection_ray = Ray::new(*hit.pos() + reflection_dir * EPSILON, reflection_dir);

            scene
                .calc_ray(&reflection_ray, recurse_power * k_r, recurse_depth + 1)
                .0
                * k_r
        } else {
            Color::new_black()
        };

        let trans_part = if k_t > MIN_RECURSE_COEFFICIENT {
            let refracted_dir = (1. / self.refractive_index) * view_ray.direction()
                - (cos_t_t - (1. / self.refractive_index) * cos_t_i) * *normal;

            let refracted_ray = Ray::new(*hit.pos() + refracted_dir * EPSILON, refracted_dir);

            scene
                .calc_ray(&refracted_ray, recurse_power * k_t, recurse_depth + 1)
                .0
                * k_t
        } else {
            Color::new_black()
        };

        refl_part + trans_part
    }
}
