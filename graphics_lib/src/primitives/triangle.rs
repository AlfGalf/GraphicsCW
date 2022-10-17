use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::Vec3;

const EPSILON: f32 = 0.00001;

#[derive(Copy, Clone, Debug)]
pub struct TrianglePrimitive {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    n: Vec3,
    d: f32,
    material: usize,
    node_index: usize,
}

impl TrianglePrimitive {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: usize) -> TrianglePrimitive {
        let normal = (c - a).cross(b - a).normalize();
        TrianglePrimitive {
            a,
            b,
            c,
            n: normal,
            d: a.dot(normal),
            material,
            node_index: 0,
        }
    }
}

impl BHShape for TrianglePrimitive {
    fn set_bh_node_index(&mut self, n: usize) {
        self.node_index = n
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Bounded for TrianglePrimitive {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(
            self.a.min(self.b).min(self.c),
            self.a.max(self.b).max(self.c),
        )
    }
}

impl Primitive for TrianglePrimitive {
    fn get_material(&self) -> usize {
        self.material
    }

    fn intersection(&self, ray: &Ray) -> Option<Hit> {
        let p0 = self.a;
        let p1 = self.b;
        let p2 = self.c;

        let normal = self.n;

        if ray.direction.dot(normal).abs() < EPSILON {
            return None;
        }

        let d = self.d;

        let t = (d - normal.dot(ray.position)) / normal.dot(ray.direction);

        let p = ray.position + t * ray.direction;

        let v0 = (p - p0).cross(p1 - p0).dot(normal);
        let v1 = (p - p1).cross(p2 - p1).dot(normal);
        let v2 = (p - p2).cross(p0 - p2).dot(normal);

        if v0 >= -EPSILON && v1 >= -EPSILON && v2 >= -EPSILON {
            Some(Hit::new(p, normal, t, Box::new(*self)))
        } else {
            None
        }
    }

    fn clone_dyn(&self) -> Box<dyn Primitive + Sync> {
        Box::new(self.clone())
    }
}
