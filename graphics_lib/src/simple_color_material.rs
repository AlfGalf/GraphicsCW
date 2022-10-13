use crate::color::Color;
use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;

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
    fn compute_once(&self, ray: &Ray, hit: &Hit) -> Color {
        self.color
    }
}
