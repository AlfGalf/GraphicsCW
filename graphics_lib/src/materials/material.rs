use crate::color::Color;
use crate::hit::Hit;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;
use std::fmt::Debug;

// Trait for materials to adhere to
pub trait Material: Debug {
    fn compute<'a>(
        &'a self,
        view_ray: Ray,
        hit: &'a Hit,
        ambient: Color,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Color;

    fn update_mat_index(&mut self, i: usize);

    fn get_mat_index(&self) -> usize;

    // Finds a list of photons resulting from a photon hitting this material
    fn compute_photon(
        &self,
        view_ray: Ray,
        hit: &Hit,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
        light_index: usize,
    ) -> Vec<Photon>;

    // Specifies if this material needs caustic photons
    fn needs_caustic(&self) -> bool;

    // Finds the outgoing ray of a caustic photon hitting this object
    fn compute_caustic_ray(
        &self,
        view_ray: Ray,
        hit: &Hit,
        scene: &Scene,
        recurse_depth: usize,
        light_index: usize,
        color: Color,
    ) -> Option<Photon>;
}
