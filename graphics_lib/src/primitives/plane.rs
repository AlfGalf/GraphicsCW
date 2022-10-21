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
    normal: Vec3,
    d: f32,
    material: usize,
    node_index: usize,
}

impl PlanePrimitive {
    pub fn new(point: Vec3, normal: Vec3, material: usize) -> Self {
        Self {
            normal,
            d: normal.dot(point),
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
        if ray.direction().dot(self.normal).abs() < EPSILON {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.position())) / self.normal.dot(ray.direction());

        let p = ray.position() + t * ray.direction();

        // TODO: Make this only intersect if going through normal side
        // TODO: Make hit record which way through material it went?

        Some(Hit::new(p, self.normal, t, Box::new(*self)))
    }

    fn clone_dyn(&self) -> Box<dyn Primitive + Sync> {
        Box::new(self.clone())
    }
}
