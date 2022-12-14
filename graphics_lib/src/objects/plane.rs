use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::plane::PlanePrimitive;
use crate::primitives::primitive::Primitive;
use crate::scene::Scene;
use glam::{DAffine3, DVec3};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Plane {
    point: DVec3,
    normal: DVec3,
    material: usize,
    csg_index: usize,
}

impl Plane {
    pub fn new(point: DVec3, normal: DVec3, material: usize) -> Plane {
        Plane {
            point,
            normal: normal.normalize(),
            material,
            csg_index: 0,
        }
    }
}

impl Object for Plane {
    fn apply_transform(&mut self, t: &DAffine3) {
        self.point = t.transform_point3(self.point);
        self.normal = t.transform_vector3(self.normal).normalize();
    }

    fn get_material(&self, _: &Hit) -> usize {
        self.material
    }

    fn set_csg_index(&mut self, csg_index: usize) {
        self.csg_index = csg_index
    }

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>> {
        vec![Box::new(PlanePrimitive::new(
            self.point,
            self.normal,
            obj_index,
            self.csg_index,
        ))]
    }

    fn filter_hits(&self, hits: Vec<Hit>, _: usize) -> Vec<Hit> {
        hits
    }

    // Cannot find bounds for a plane as it is infinite
    // If a plane is wanted for part of a caustic object, it vcan be wrapped in a intersection CSG
    // with a finite object
    fn get_caustic_bounds(&self) -> (DVec3, DVec3) {
        (
            DVec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            DVec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        )
    }

    fn needs_caustic(&self, scene: &Scene) -> bool {
        scene.material_needs_caustic(self.material)
    }
}
