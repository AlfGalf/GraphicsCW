use crate::hit::Hit;
use crate::ray::Ray;
use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use std::fmt::Debug;

// BHShape and Bounded are traits required for the BVH data structure
pub trait Primitive: BHShape + Bounded + Debug {
    // Gets the scene index of the object this originated from
    fn get_object(&self) -> usize;

    // Gets the csg index of the leaf object this originated from in a CSG tree
    fn get_csg_index(&self) -> usize;

    // Finds all intersections of this object and a ray
    fn intersection(&self, ray: &Ray) -> Vec<Hit>;
}
