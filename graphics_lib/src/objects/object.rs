use crate::materials::material::Material;
use crate::primitives::primitive::Primitive;
use glam::Affine3A;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Object: Debug {
    fn apply_transform(&mut self, t: &Affine3A);

    fn get_material(&self) -> Arc<dyn Material + Sync + Send>;

    fn primitives(&self) -> Vec<Box<dyn Primitive + Sync + Send>>;
}
