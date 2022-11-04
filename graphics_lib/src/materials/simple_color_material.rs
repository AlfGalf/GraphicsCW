use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone)]
pub struct SimpleColorMaterial {
    color: Color,
}

impl SimpleColorMaterial {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for SimpleColorMaterial {
    fn compute(&self, _: &Ray, _: &Hit, _: Color, _: &Scene, _: usize, _: Color) -> Color {
        self.color
    }
}
