use glam::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    position: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(position: Vec3, direction: Vec3) -> Ray {
        Ray {
            position,
            direction: direction.normalize(),
        }
    }

    pub fn bvh_ray(&self) -> bvh::ray::Ray {
        bvh::ray::Ray::new(self.position - 100. * self.direction, self.direction)
    }

    pub fn position(&self) -> Vec3 {
        self.position.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.direction.clone()
    }
}
