use crate::primitives::primitive::Primitive;
use glam::Vec3;

#[derive(Clone)]
pub struct Hit {
    pos: Vec3,
    normal: Vec3,
    distance: f32,
    obj_index: usize,
    correct_dir: bool,
    csg_index: usize,
}

impl Hit {
    pub fn new(
        pos: Vec3,
        normal: Vec3,
        distance: f32,
        correct_dir: bool,
        obj_index: usize,
        csg_index: usize,
    ) -> Hit {
        Hit {
            pos,
            normal: normal.normalize(),
            distance,
            correct_dir,
            obj_index,
            csg_index,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn get_object_index(&self) -> usize {
        self.obj_index
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_dir(&self) -> bool {
        self.correct_dir
    }

    pub fn get_csg_index(&self) -> usize {
        self.csg_index
    }

    pub fn flip(&mut self) {
        self.correct_dir = !self.correct_dir;
        self.normal = -self.normal;
    }
}
