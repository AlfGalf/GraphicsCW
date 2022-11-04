use crate::color::Color;
use crate::constants::EPSILON;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

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
        let cos_t_i = view_ray.direction().dot(*normal);
        let cos_t_t = (1. - (1. / self.refractive_index.powi(2)) * (1. - cos_t_i.powi(2))).sqrt();
        let refracted_dir = (1. / self.refractive_index) * view_ray.direction()
            - (cos_t_t - (1. / self.refractive_index) * cos_t_i) * *normal;

        let refracted_ray = Ray::new(*hit.pos() + refracted_dir * EPSILON, refracted_dir);
        //
        // let internal_hit = scene.intersection(&refracted_ray).filter(|h| h.get_object().get_material() == )
        //
        scene
            .calc_ray(&refracted_ray, recurse_power, recurse_depth + 1)
            .0
    }
    fn clone_dyn(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
