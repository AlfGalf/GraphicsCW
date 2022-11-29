use crate::hit::Hit;
use crate::ray::Ray;
use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use std::fmt::Debug;

pub trait Primitive: BHShape + Bounded + Debug {
    fn get_object(&self) -> usize;

    fn get_csg_index(&self) -> usize;

    fn intersection(&self, ray: &Ray) -> Vec<Hit>;
}
