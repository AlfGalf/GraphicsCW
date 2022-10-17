use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::sphere::SpherePrimitive;
use glam::{Affine3A, Vec3};

#[derive(Debug)]
pub struct Sphere<M: Material> {
    center: Vec3,
    rad: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, rad: f32, material: M) -> Sphere<M> {
        Sphere {
            center,
            rad,
            material,
        }
    }
}

impl<M: Material + Clone> Object for Sphere<M> {
    fn apply_transform(self: &mut Sphere<M>, t: &Affine3A) {
        self.center = t.transform_point3(self.center);
    }

    fn get_material(&self) -> Box<&dyn Material> {
        Box::new(&self.material)
    }

    fn primitives(&self, index: usize) -> Vec<Box<dyn Primitive + Sync>> {
        vec![Box::new(SpherePrimitive::new(self.center, self.rad, index))]
    }
}
