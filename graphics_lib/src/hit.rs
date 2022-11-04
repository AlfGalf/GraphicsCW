use crate::primitives::primitive::Primitive;
use glam::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Hit {
    pos: Vec3,
    normal: Vec3,
    distance: f32,
    object: Box<dyn Primitive + Sync + Send>,
}

impl Hit {
    pub fn new(
        pos: Vec3,
        normal: Vec3,
        distance: f32,
        object: Box<dyn Primitive + Sync + Send>,
    ) -> Hit {
        Hit {
            pos,
            normal: normal.normalize(),
            distance,
            object,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_object(&self) -> &Box<dyn Primitive + Sync + Send> {
        &self.object
    }
}
