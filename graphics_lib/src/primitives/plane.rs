use crate::constants::{EPSILON, SCENE_BOUNDS};
use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::{DVec3, Vec3};

#[derive(Debug, Clone)]
pub struct PlanePrimitive {
    normal: DVec3,
    d: f64,
    node_index: usize,
    obj_index: usize,
    csg_index: usize,
}

impl PlanePrimitive {
    pub fn new(point: DVec3, normal: DVec3, obj_index: usize, csg_index: usize) -> Self {
        Self {
            normal,
            d: normal.dot(point),
            node_index: 0,
            obj_index,
            csg_index,
        }
    }
}

impl BHShape for PlanePrimitive {
    fn set_bh_node_index(&mut self, n: usize) {
        self.node_index = n
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

// Cannot bound a plane, so give infinite bounds
impl Bounded for PlanePrimitive {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(
            Vec3::new(-SCENE_BOUNDS, -SCENE_BOUNDS, -SCENE_BOUNDS),
            Vec3::new(SCENE_BOUNDS, SCENE_BOUNDS, SCENE_BOUNDS),
        )
    }
}

impl Primitive for PlanePrimitive {
    fn get_object(&self) -> usize {
        self.obj_index
    }

    fn get_csg_index(&self) -> usize {
        self.csg_index
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        if ray.direction().dot(self.normal).abs() < EPSILON {
            return vec![];
        }

        let t = (self.d - self.normal.dot(ray.position())) / self.normal.dot(ray.direction());

        let p = ray.position() + t * ray.direction();

        vec![Hit::new(
            p,
            self.normal,
            t,
            self.normal.dot(ray.direction()) < 0.,
            self.obj_index,
            self.csg_index,
        )]
    }
}
