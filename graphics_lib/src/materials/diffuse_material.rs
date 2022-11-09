use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone)]
pub struct DiffuseMaterial {
    mat_index: usize,
}

impl DiffuseMaterial {
    pub fn new() -> DiffuseMaterial {
        DiffuseMaterial { mat_index: 0 }
    }
}

impl Material for DiffuseMaterial {
    fn compute<'a>(&self, _: Ray, hit: &Hit, _: Color, scene: &Scene, _: usize, _: Color) -> Color {
        scene
            .get_lights()
            .iter()
            .fold(Color::new_black(), |c, light| {
                let intensity = light.get_intensity(*hit.pos(), scene);

                let dir = light.get_direction(*hit.pos());

                let diffuse = hit.normal().dot(-dir).max(0.);

                c + intensity * diffuse
            })
    }

    fn update_mat_index(&mut self, i: usize) {
        self.mat_index = i
    }

    fn get_mat_index(&self) -> usize {
        self.mat_index
    }
}
