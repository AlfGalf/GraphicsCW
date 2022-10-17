use crate::objects::object::Object;
use glam::Vec3;

#[derive(Clone)]
pub struct Hit<'a> {
    pub pos: Vec3,
    pub normal: Vec3,
    distance: f32,
    object: Box<&'a dyn Object>,
}

impl Hit<'_> {
    pub fn new<'a>(pos: Vec3, normal: Vec3, distance: f32, object: Box<&'a dyn Object>) -> Hit<'a> {
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

    pub fn get_object(&self) -> &Box<&dyn Object> {
        &self.object
    }
}
