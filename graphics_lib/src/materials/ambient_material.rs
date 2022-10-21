use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Clone, Debug)]
pub struct AmbientMaterial {}

impl AmbientMaterial {
    pub(crate) fn new() -> AmbientMaterial {
        AmbientMaterial {}
    }
}

impl Material for AmbientMaterial {
    fn compute(&self, _: &Ray, _: &Hit, ambient: Color, _: &Scene, _: usize, _: Color) -> Color {
        ambient
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}
