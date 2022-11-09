use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Clone, Debug)]
pub struct AmbientMaterial {
    mat_index: usize,
}

impl AmbientMaterial {
    pub(crate) fn new() -> AmbientMaterial {
        AmbientMaterial { mat_index: 0 }
    }
}

impl Material for AmbientMaterial {
    fn compute(&self, _: Ray, _: &Hit, ambient: Color, _: &Scene, _: usize, _: Color) -> Color {
        ambient
    }

    fn update_mat_index(&mut self, i: usize) {
        self.mat_index = i
    }

    fn get_mat_index(&self) -> usize {
        self.mat_index
    }
}
