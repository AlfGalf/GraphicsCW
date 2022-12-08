use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;

// This material is for debugging purposes and visualises the normals of surfaces
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
        ) * 0.2
    }

    fn compute_photon(
        &self,
        _: Ray,
        _: &Hit,
        _: &Scene,
        _: usize,
        _: Color,
        _: usize,
    ) -> Vec<Photon> {
        // Note: This is a debugging material so does not support this
        vec![]
    }

    fn needs_caustic(&self) -> bool {
        false
    }

    fn compute_caustic_ray(
        &self,
        _view_ray: Ray,
        _hit: &Hit,
        _scene: &Scene,
        _recurse_depth: usize,
        _light_index: usize,
        _: Color,
    ) -> Option<Photon> {
        // This is a debugging material so does nto support this
        None
    }
}
