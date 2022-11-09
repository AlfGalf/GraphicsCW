use crate::primitives::primitive::Primitive;
use glam::Affine3A;
use std::fmt::Debug;

pub trait Object: Debug {
    fn apply_transform(&mut self, t: &Affine3A);

    fn get_material(&self) -> usize;

    fn primitives(&self) -> Vec<Box<dyn Primitive + Sync + Send>>;
}
