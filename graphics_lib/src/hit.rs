use crate::primitives::primitive::Primitive;
use glam::Vec3;

#[derive(Clone)]
pub struct Hit<'scene> {
    pos: Vec3,
    normal: Vec3,
    distance: f32,
    object: &'scene (dyn Primitive + Sync + Send),
    correct_dir: bool,
}

impl<'scene> Hit<'scene> {
    pub fn new(
        pos: Vec3,
        normal: Vec3,
        distance: f32,
        object: &'scene (dyn Primitive + Sync + Send + 'scene),
        correct_dir: bool,
    ) -> Hit<'scene> {
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

    pub fn get_object(&self) -> &'scene (dyn Primitive + Sync + Send) {
        self.object
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_dir(&self) -> bool {
        self.correct_dir
    }
}
