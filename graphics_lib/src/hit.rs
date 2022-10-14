use glam::Vec3;

#[derive(Copy, Clone)]
pub struct Hit {
    pub pos: Vec3,
    pub normal: Vec3,
    distance: f32,
}

impl Hit {
    pub fn new(pos: Vec3, normal: Vec3, distance: f32) -> Self {
        Hit {
            pos,
            normal: normal.normalize(),
            distance,
        }
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }
}
