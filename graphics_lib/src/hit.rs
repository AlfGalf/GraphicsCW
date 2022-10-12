use glam::Vec3;

#[derive(Copy, Clone)]
pub struct Hit {
    pub pos: Vec3,
    pub normal: Vec3,
}

impl Hit {
    pub fn new(pos: Vec3, normal: Vec3) -> Self {
        Hit { pos, normal }
    }
}
