use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone)]
pub struct SimpleColorMaterial {
    color: Color,
    mat_index: usize,
}

impl SimpleColorMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            mat_index: 0,
        }
    }
}

impl Material for SimpleColorMaterial {
    fn compute(&self, _: Ray, _: &Hit, _: Color, _: &Scene, _: usize, _: Color) -> Color {
        self.color
    }

    fn update_mat_index(&mut self, i: usize) {
        self.mat_index = i
    }

    fn get_mat_index(&self) -> usize {
        self.mat_index
    }
}
