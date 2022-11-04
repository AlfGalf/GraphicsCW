use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::sphere::SpherePrimitive;
use glam::{Affine3A, Vec3};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    rad: f32,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Vec3, rad: f32, material: Arc<dyn Material + Sync + Send>) -> Sphere {
        Sphere {
            center,
            rad,
            material,
        }
    }
}

impl Object for Sphere {
    fn apply_transform(self: &mut Sphere, t: &Affine3A) {
        self.center = t.transform_point3(self.center);
    }

    fn get_material(&self) -> Arc<dyn Material + Sync + Send> {
        self.material.clone()
    }

    fn primitives(&self) -> Vec<Box<dyn Primitive + Sync + Send>> {
        vec![Box::new(SpherePrimitive::new(
            self.center,
            self.rad,
            self.material.clone(),
        ))]
    }
}
