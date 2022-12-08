use glam::Vec3;

// represents a light ray
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    position: Vec3,
    direction: Vec3,
}

impl Ray {
    pub(crate) fn new(position: Vec3, direction: Vec3) -> Ray {
        Ray {
            position,
            direction: direction.normalize(),
        }
    }

    pub(crate) fn bvh_ray(&self) -> bvh::ray::Ray {
        // Start the BVH ray far back so it passes through all the objects, even those
        // before the start of the ray
        bvh::ray::Ray::new(self.position - 1000. * self.direction, self.direction)
    }

    pub(crate) fn position(&self) -> Vec3 {
        self.position
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.direction
    }
}
