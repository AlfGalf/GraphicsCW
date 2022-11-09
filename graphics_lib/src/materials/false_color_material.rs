use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone)]
pub struct FalseColorMaterial {
    mat_index: usize,
}

impl FalseColorMaterial {
    pub fn new() -> Self {
        FalseColorMaterial { mat_index: 0 }
    }
}
impl Material for FalseColorMaterial {
    fn compute(&self, _: Ray, hit: &Hit, _: Color, _: &Scene, _: usize, _: Color) -> Color {
        Color::new(
            (hit.normal().x + 1.0) * 0.5,
            (hit.normal().y + 1.0) * 0.5,
            (hit.normal().z + 1.0) * 0.5,
        )
    }

    fn update_mat_index(&mut self, i: usize) {
        self.mat_index = i
    }

    fn get_mat_index(&self) -> usize {
        self.mat_index
    }
}
