use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
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
    fn compute_once(&self, _: &Ray, _: &Hit, _: Color) -> Color {
        self.color
    }

    fn compute_per_light(&self, _: &Ray, _: &Hit, _: &Vec3, _: Color) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
