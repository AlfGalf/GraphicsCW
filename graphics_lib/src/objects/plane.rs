use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::primitives::plane::PlanePrimitive;
use crate::primitives::primitive::Primitive;
use glam::{Affine3A, Vec3};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Plane<M: Material> {
    point: Vec3,
    normal: Vec3,
    material: M,
}

impl<M: Material> Plane<M> {
    pub fn new(point: Vec3, normal: Vec3, material: M) -> Plane<M> {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl<M: Material> Object for Plane<M> {
    fn apply_transform(&mut self, t: &Affine3A) {
        self.point = t.transform_point3(self.point);
        self.normal = t.transform_vector3(self.normal);
    }

    fn get_material(&self) -> Box<&dyn Material> {
        Box::new(&self.material)
    }

    fn primitives(&self, index: usize) -> Vec<Box<dyn Primitive + Sync>> {
        vec![Box::new(PlanePrimitive::new(
            self.point,
            self.normal,
            index,
        ))]
    }
}
