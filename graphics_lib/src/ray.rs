use glam::DVec3;

// represents a light ray
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    position: DVec3,
    direction: DVec3,
}

impl Ray {
    pub(crate) fn new(position: DVec3, direction: DVec3) -> Ray {
        Ray {
            position,
            direction: direction.normalize(),
        }
    }

    pub(crate) fn bvh_ray(&self) -> bvh::ray::Ray {
        // Start the BVH ray far back so it passes through all the objects, even those
        // before the start of the ray
        bvh::ray::Ray::new(
            (self.position - 1000. * self.direction).as_vec3(),
            self.direction.as_vec3(),
        )
    }

    pub(crate) fn position(&self) -> DVec3 {
        self.position
    }

    pub(crate) fn direction(&self) -> DVec3 {
        self.direction
    }
}
