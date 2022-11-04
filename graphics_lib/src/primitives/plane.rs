use crate::hit::Hit;
use crate::materials::material::Material;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::Vec3;
use std::sync::Arc;

const SCENE_BOUNDS: f32 = 1.0E10;
const EPSILON: f32 = 0.00001;

#[derive(Debug, Clone)]
pub struct PlanePrimitive {
    normal: Vec3,
    d: f32,
    material: Arc<dyn Material + Sync + Send>,
    node_index: usize,
}

impl PlanePrimitive {
    pub fn new(point: Vec3, normal: Vec3, material: Arc<dyn Material + Sync + Send>) -> Self {
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
    fn get_material(&self) -> Arc<dyn Material + Sync + Send> {
        self.material.clone()
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        if ray.direction().dot(self.normal).abs() < EPSILON {
            return vec![];
        }

        let t = (self.d - self.normal.dot(ray.position())) / self.normal.dot(ray.direction());

        let p = ray.position() + t * ray.direction();

        // TODO: Make this only intersect if going through normal side
        // TODO: Make hit record which way through material it went?

        vec![Hit::new(
            p,
            self.normal,
            t,
            Box::new(self),
            self.normal.dot(ray.direction()) < 0.,
        )]
    }

    fn clone_dyn(&self) -> Box<dyn Primitive + Sync + Send> {
        Box::new(self.clone())
    }
}
