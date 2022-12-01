use crate::color::Color;
use crate::hit::Hit;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;
use std::fmt::Debug;

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

    // fn compute_photon(
    //     &self,
    //     view_ray: Ray,
    //     hit: &Hit,
    //     scene: &Scene,
    //     recurse_depth: usize,
    //     recurse_power: Color,
    // ) -> Vec<Photon>;
}
