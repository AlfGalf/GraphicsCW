use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::scene::Scene;
use std::fmt::Debug;

pub trait Material: Debug {
    fn compute(
        &self,
        view_ray: &Ray,
        hit: &Hit,
        ambient: Color,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Color;
}
