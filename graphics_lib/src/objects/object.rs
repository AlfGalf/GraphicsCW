use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use glam::Affine3A;
use std::fmt::Debug;

pub trait Object: Debug {
    fn intersection(&self, r: &Ray) -> Option<Hit>;
    fn apply_transform(&mut self, t: &Affine3A);

    fn get_material(&self) -> Box<&dyn Material>;
}
