use crate::primitives::primitive::Primitive;
use glam::Vec3;

#[derive(Clone)]
pub struct Hit {
    pub pos: Vec3,
    pub normal: Vec3,
    distance: f32,
    object: Box<dyn Primitive + Sync>,
}

impl Hit {
    pub fn new(pos: Vec3, normal: Vec3, distance: f32, object: Box<dyn Primitive + Sync>) -> Hit {
        Hit {
            pos,
            normal: normal.normalize(),
            distance,
            object,
        }
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_object(&self) -> &Box<dyn Primitive + Sync> {
        &self.object
    }
}
