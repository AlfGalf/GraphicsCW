use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use glam::Affine3A;
use std::fmt::Debug;

pub trait Object: Debug {
    fn apply_transform(&mut self, t: &Affine3A);

    fn get_material(&self) -> usize;

    fn set_csg_index(&mut self, csg_index: usize);

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>>;

    fn filter_hits(&self, hits: Vec<Hit>, index: usize) -> Vec<Hit>;
}
