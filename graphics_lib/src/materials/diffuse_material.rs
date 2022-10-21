use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone)]
pub struct DiffuseMaterial {}

impl DiffuseMaterial {
    pub fn new() -> DiffuseMaterial {
        DiffuseMaterial {}
    }
}

impl Material for DiffuseMaterial {
    fn compute(&self, _: &Ray, hit: &Hit, _: Color, scene: &Scene, _: usize, _: Color) -> Color {
        scene.lights.iter().fold(Color::new_black(), |c, light| {
            let intensity = light.get_intensity(hit.pos(), scene);

            let dir = light.get_direction(hit.pos());

            let diffuse = hit.normal().dot(-dir).max(0.);

            c + intensity * diffuse
        })
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}
