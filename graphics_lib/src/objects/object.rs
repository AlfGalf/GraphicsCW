use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::scene::Scene;
use glam::{Affine3A, Vec3};
use std::fmt::Debug;

// This represents an object in the scene
pub trait Object: Debug {
    fn apply_transform(&mut self, t: &Affine3A);

    fn get_material(&self, hit: &Hit) -> usize;

    fn set_csg_index(&mut self, csg_index: usize);

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>>;

    fn filter_hits(&self, hits: Vec<Hit>, index: usize) -> Vec<Hit>;

    fn get_caustic_bounds(&self) -> (Vec3, Vec3);

    fn needs_caustic(&self, scene: &Scene) -> bool;
}
