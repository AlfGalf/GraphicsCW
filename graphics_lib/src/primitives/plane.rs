use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::Vec3;

const SCENE_BOUNDS: f32 = 1.0E10;
const EPSILON: f32 = 0.00001;

#[derive(Debug, Clone)]
pub struct PlanePrimitive {
    normal: Vec3,
    d: f32,
    material_index: usize,
    node_index: usize,
    obj_index: usize,
    csg_index: usize,
}

impl PlanePrimitive {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        material_index: usize,
        obj_index: usize,
        csg_index: usize,
    ) -> Self {
        Self {
            normal,
            d: normal.dot(point),
            material_index,
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
