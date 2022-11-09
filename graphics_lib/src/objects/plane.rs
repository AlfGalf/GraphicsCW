use crate::objects::object::Object;
use crate::primitives::plane::PlanePrimitive;
use crate::primitives::primitive::Primitive;
use glam::{Affine3A, Vec3};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material: usize,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: usize) -> Plane {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Object for Plane {
    fn apply_transform(&mut self, t: &Affine3A) {
        self.point = t.transform_point3(self.point);
        self.normal = t.transform_vector3(self.normal);
    }

    fn get_material(&self) -> usize {
        self.material
    }

    fn primitives(&self) -> Vec<Box<dyn Primitive + Sync + Send>> {
        vec![Box::new(PlanePrimitive::new(
            self.point,
            self.normal,
            self.material,
        ))]
    }
}
