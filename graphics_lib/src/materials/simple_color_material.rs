use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;

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

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}