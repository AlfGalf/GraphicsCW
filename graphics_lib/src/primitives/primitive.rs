use crate::hit::Hit;
use crate::ray::Ray;
use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use std::fmt::Debug;

pub trait Primitive: BHShape + Bounded + Debug {
    fn get_material(&self) -> usize;
    fn intersection(&self, ray: &Ray) -> Option<Hit>;
    fn clone_dyn(&self) -> Box<dyn Primitive + Sync>;
}

impl Clone for Box<dyn Primitive + Sync> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
