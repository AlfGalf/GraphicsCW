use glam::Vec3;

pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(position: Vec3, direction: Vec3) -> Ray {
        Ray {
            position,
            direction: direction.normalize(),
        }
    }
}
