use crate::color::Color;
use crate::constants::{EPSILON, MAX_RECURSE_DEPTH, MIN_RECURSE_COEFFICIENT};
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;
use std::borrow::Borrow;
use std::ptr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransparentMaterial {
    refractive_index: f32,
}

impl TransparentMaterial {
    pub fn new(refractive_index: f32) -> Self {
        TransparentMaterial { refractive_index }
    }

    fn calc_internal_ray(
        &self,
        ray: &Ray,
        scene: &Scene,
        recurse_power: Color,
        recurse_depth: usize,
    ) -> Color {
        let mut intersections = scene
            .intersection(ray)
            .filter(|h| {
                (!h.get_dir())
                    && h.get_distance() > 0.
                    && ptr::eq(
                        h.get_object().get_material().as_ref(),
                        self as &(dyn Material + Send + Sync),
                    )
            })
            .collect::<Vec<Hit>>();

        intersections.sort_by(|l, r| l.get_distance().partial_cmp(&r.get_distance()).unwrap());

        if let Some(hit) = intersections.first() {
            let (trans_ray, trans_coeff, refl_ray, refl_coeff) =
                self.find_rays(*hit.normal(), ray.direction(), *hit.pos(), false);

            let refl_power = recurse_power * refl_coeff;
            let trans_power = recurse_power * trans_coeff;

            let refl_part = if refl_power.max_val() > MIN_RECURSE_COEFFICIENT
                && recurse_depth < MAX_RECURSE_DEPTH
            {
                self.calc_internal_ray(&refl_ray, scene, refl_power, recurse_depth + 1) * refl_coeff
            } else {
                Color::new_black()
            };

            let trans_part = if trans_power.max_val() > MIN_RECURSE_COEFFICIENT
                && recurse_depth < MAX_RECURSE_DEPTH
            {
                scene
                    .calc_ray(&trans_ray.unwrap(), trans_power, recurse_depth + 1)
                    .0
                    * trans_coeff
            } else {
                Color::new_black()
            };

            refl_part + trans_part
        } else {
            scene.calc_ray(&ray, recurse_power, recurse_depth + 1).0
        }
    }

    fn find_rays(
        &self,
        normal: Vec3,
        incidence: Vec3,
        pos: Vec3,
        going_in: bool,
    ) -> (Option<Ray>, f32, Ray, f32) {
        let refr_index = if going_in {
            self.refractive_index
        } else {
            1. / self.refractive_index
        };

        let reflection_dir = incidence - 2. * (incidence.dot(normal)) * normal;
        let reflection_dir = reflection_dir.normalize();
        let reflection_ray = Ray::new(pos + reflection_dir * EPSILON, reflection_dir);

        let cos_t_i = (normal).dot(-incidence);
        let test_part = 1. - (1. / refr_index.powi(2)) * (1. - cos_t_i.powi(2));
        if test_part < 0. {
            return (None, 0., reflection_ray, 1.);
        }

        let cos_t_t = test_part.sqrt();

        let r_par = (refr_index * cos_t_i - cos_t_t) / (refr_index * cos_t_i + cos_t_t);
        let r_per = (cos_t_i - refr_index * cos_t_t) / (cos_t_i + refr_index * cos_t_t);

        let k_r = (r_par.powi(2) + r_per.powi(2)) / 2.;
        let k_r = k_r.min(1.).max(0.);
        let k_t = 1. - k_r;

        let refracted_dir =
            (1. / refr_index) * (incidence) - (cos_t_t - (1. / refr_index) * cos_t_i) * normal;

        let refracted_ray = Ray::new(pos + refracted_dir * EPSILON, refracted_dir);

        (Some(refracted_ray), k_t, reflection_ray, k_r)
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
        let (trans_ray, trans_coeff, refl_ray, refl_coeff) =
            self.find_rays(*hit.normal(), view_ray.direction(), *hit.pos(), true);

        let refl_power = recurse_power * refl_coeff;
        let trans_power = recurse_power * trans_coeff;

        let refl_part = if refl_power.max_val() > MIN_RECURSE_COEFFICIENT
            && recurse_depth < MAX_RECURSE_DEPTH
        {
            scene.calc_ray(&refl_ray, refl_power, recurse_depth + 1).0 * refl_coeff
        } else {
            Color::new_black()
        };

        let trans_part = if trans_power.max_val() > MIN_RECURSE_COEFFICIENT
            && recurse_depth < MAX_RECURSE_DEPTH
        {
            self.calc_internal_ray(&trans_ray.unwrap(), scene, trans_power, recurse_depth)
                * trans_coeff
        } else {
            Color::new_black()
        };

        refl_part + trans_part
    }
}
