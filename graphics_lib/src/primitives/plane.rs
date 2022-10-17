use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::Vec3;

const SCENE_BOUNDS: f32 = 1.0E10;
const EPSILON: f32 = 0.00001;

#[derive(Debug, Copy, Clone)]
pub struct PlanePrimitive {
    point: Vec3,
    normal: Vec3,
    d: f32,
    material: usize,
    node_index: usize,
}

impl PlanePrimitive {
    pub fn new(point: Vec3, normal: Vec3, material: usize) -> Self {
        Self {
            point,
            normal,
            d: 0.0,
            material,
            node_index: 0,
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
    fn get_material(&self) -> usize {
        self.material
    }

    fn intersection(&self, ray: &Ray) -> Option<Hit> {
        let normal = self.normal;

        if ray.direction.dot(normal).abs() < EPSILON {
            return None;
        }

        let d = self.normal.dot(self.point);

        let t = (d - normal.dot(ray.position)) / normal.dot(ray.direction);

        let p = ray.position + t * ray.direction;

        Some(Hit::new(p, normal, t, Box::new(*self)))
    }

    fn clone_dyn(&self) -> Box<dyn Primitive + Sync> {
        Box::new(self.clone())
    }
}
