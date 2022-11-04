use crate::primitives::primitive::Primitive;
use glam::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Hit<'a> {
    pos: Vec3,
    normal: Vec3,
    distance: f32,
    object: Box<&'a (dyn Primitive + Sync + Send)>,
    correct_dir: bool,
}

impl<'a> Hit<'a> {
    pub fn new(
        pos: Vec3,
        normal: Vec3,
        distance: f32,
        object: Box<&'a (dyn Primitive + Sync + Send)>,
        correct_dir: bool,
    ) -> Hit {
        Hit {
            pos,
            normal: normal.normalize(),
            distance,
            object,
            correct_dir,
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

    pub fn get_object(&self) -> Box<&(dyn Primitive + Sync + Send)> {
        Box::new(*self.object)
    }

    pub fn get_dir(&self) -> bool {
        self.correct_dir
    }
}
