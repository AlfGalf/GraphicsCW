use crate::hit::Hit;
use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::ray::Ray;
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
    fn intersection(&self, ray: &Ray) -> Option<Hit> {
        let epsilon = 0.00001;

        let normal = self.normal;

        if ray.direction.dot(normal).abs() < epsilon {
            return None;
        }

        let d = self.normal.dot(self.point);

        let t = (d - normal.dot(ray.position)) / normal.dot(ray.direction);

        let p = ray.position + t * ray.direction;

        Some(Hit::new(p, normal, t, Box::new(self)))
    }

    fn apply_transform(&mut self, t: &Affine3A) {
        self.point = t.transform_point3(self.point);
        self.normal = t.transform_vector3(self.normal);
    }

    fn get_material(&self) -> Box<&dyn Material> {
        Box::new(&self.material)
    }
}
