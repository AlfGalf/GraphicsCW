use crate::color::Color;
use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct FalseColorMaterial {}

impl Material for FalseColorMaterial {
    fn compute_once(&self, ray: &Ray, hit: &Hit) -> Color {
        Color {
            red: (hit.normal.x + 1.0) * 0.5,
            green: (hit.normal.y + 1.0) * 0.5,
            blue: (hit.normal.z + 1.0) * 0.5,
        }
    }
}
