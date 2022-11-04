use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;
use std::fmt::Debug;

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
    fn compute(
        &self,
        view_ray: &Ray,
        hit: &Hit,
        _: Color,
        scene: &Scene,
        _: usize,
        _: Color,
    ) -> Color {
        let reflection_dir: Vec3 =
            view_ray.direction() - 2. * (view_ray.direction().dot(*hit.normal())) * *hit.normal();

        let reflection_dir = reflection_dir.normalize();

        scene.lights.iter().fold(Color::new_black(), |c, light| {
            let intensity = light.get_intensity(hit.pos(), scene);
            let dir = light.get_direction(hit.pos());

            let specular = reflection_dir.dot(-dir).powi(self.power as i32).max(0.);

            c + intensity * specular
        })
    }

    fn clone_dyn(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
