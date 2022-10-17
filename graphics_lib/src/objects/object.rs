use crate::materials::material::Material;
use crate::primitives::primitive::Primitive;
use glam::Affine3A;
use std::fmt::Debug;

pub trait Object: Debug {
    fn apply_transform(&mut self, t: &Affine3A);

    fn get_material(&self) -> Box<&dyn Material>;

    fn primitives(&self, index: usize) -> Vec<Box<dyn Primitive + Sync>>;
}
