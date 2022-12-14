use crate::constants::EPSILON;
use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::DVec3;

#[derive(Clone, Debug)]
pub struct SpherePrimitive {
    center: DVec3,
    rad: f64,
    node_index: usize,
    obj_index: usize,
    csg_index: usize,
}

impl SpherePrimitive {
    pub(crate) fn new(center: DVec3, rad: f64, obj_index: usize, csg_index: usize) -> Self {
        Self {
            center,
            rad,
            node_index: 0,
            obj_index,
            csg_index,
        }
    }
}
impl BHShape for SpherePrimitive {
    fn set_bh_node_index(&mut self, n: usize) {
        self.node_index = n
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Bounded for SpherePrimitive {
    fn aabb(&self) -> AABB {
        let half = DVec3::new(self.rad, self.rad, self.rad);
        AABB::with_bounds(
            (self.center - half).as_vec3(),
            (self.center + half).as_vec3(),
        )
    }
}

impl Primitive for SpherePrimitive {
    fn get_object(&self) -> usize {
        self.obj_index
    }

    fn get_csg_index(&self) -> usize {
        self.csg_index
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let relative_position: DVec3 = ray.position() - self.center;
        let b = 2. * ray.direction().dot(relative_position);
        let c = relative_position.dot(relative_position) - self.rad * self.rad;

        let discriminant: f64 = b * b - 4. * c;

        // If discriminant sufficiently small, then hit either doesnt exist or is very close to edge
        if discriminant <= EPSILON {
            Vec::new()
        } else {
            let sqrt = discriminant.sqrt();
            let t0 = (sqrt - b) / 2.;
            let t1 = (-sqrt - b) / 2.;

            let t_first = t0.min(t1);
            let pos1 = ray.position() + ray.direction() * t_first;
            let t_second = t0.max(t1);
            let pos2 = ray.position() + ray.direction() * t_second;

            vec![
                Hit::new(
                    pos1,
                    (pos1 - self.center).normalize(),
                    t_first,
                    true,
                    self.obj_index,
                    self.csg_index,
                ),
                Hit::new(
                    pos2,
                    (pos2 - self.center).normalize(),
                    t_second,
                    false,
                    self.obj_index,
                    self.csg_index,
                ),
            ]
        }
    }
}
