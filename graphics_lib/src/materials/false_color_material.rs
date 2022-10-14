use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct FalseColorMaterial {}

impl FalseColorMaterial {
    pub fn new() -> Self {
        FalseColorMaterial {}
    }
}
impl Material for FalseColorMaterial {
    fn compute_once(&self, _: &Ray, hit: &Hit, _: Color) -> Color {
        Color::new(
            (hit.normal.x + 1.0) * 0.5,
            (hit.normal.y + 1.0) * 0.5,
            (hit.normal.z + 1.0) * 0.5,
        )
    }

    fn compute_per_light(&self, _: &Ray, _: &Hit, _: &Vec3, _: Color) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
