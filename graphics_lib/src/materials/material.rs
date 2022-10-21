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

    fn clone_dyn(&self) -> Box<dyn Material + Sync>;
}

impl Clone for Box<dyn Material + Sync> {
    fn clone(self: &Box<dyn Material + Sync>) -> Box<dyn Material + Sync> {
        self.clone_dyn()
    }
}
