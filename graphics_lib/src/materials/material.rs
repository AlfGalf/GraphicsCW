use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;
use glam::Vec3;
use std::fmt::Debug;

pub trait Material: Debug {
    fn compute_once(&self, ray: &Ray, hit: &Hit, ambient: Color) -> Color;
    fn compute_per_light(&self, viewer: &Ray, hit: &Hit, dir: &Vec3, light: Color) -> Color;

    fn clone_dyn(&self) -> Box<dyn Material + Sync>;
}

impl Clone for Box<dyn Material + Sync> {
    fn clone(self: &Box<dyn Material + Sync>) -> Box<dyn Material + Sync> {
        self.clone_dyn()
    }
}
