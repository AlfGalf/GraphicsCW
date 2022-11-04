use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Primitive: BHShape + Bounded + Debug {
    fn get_material(&self) -> Arc<dyn Material + Sync + Send>;
    fn intersection(&self, ray: &Ray) -> Option<Hit>;
    fn clone_dyn(&self) -> Box<dyn Primitive + Sync + Send>;
}

impl Clone for Box<dyn Primitive + Sync + Send> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
