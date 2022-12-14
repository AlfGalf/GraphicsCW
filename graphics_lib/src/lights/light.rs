use crate::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::DVec3;
use std::fmt::Debug;

pub trait Light: Debug {
    fn get_intensity(&self, point: DVec3, scene: &Scene, light_index: usize) -> Color;
    fn get_direction(&self, point: DVec3) -> DVec3;

    fn generate_photon_dir(&self) -> Ray;
    fn generate_caustic_dir(&self, bounds: (DVec3, DVec3)) -> Ray;

    fn get_color(&self) -> Color;
}
