use crate::hit::Hit;
use crate::materials::material::Material;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::{Mat3, Vec3};
use std::arch::aarch64::vext_f32;
use std::rc::Rc;
use std::sync::Arc;

const EPSILON: f32 = 0.00001;

#[derive(Clone, Debug)]
pub struct TrianglePrimitive {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    n: Vec3,
    an: Vec3,
    bn: Vec3,
    cn: Vec3,
    mat: Mat3,
    d: f32,
    smoothing: bool,
    material: Arc<dyn Material + Sync + Send>,
    node_index: usize,
}

impl TrianglePrimitive {
    pub fn new(
        a: Vec3,
        b: Vec3,
        c: Vec3,
        n: Vec3,
        an: Vec3,
        bn: Vec3,
        cn: Vec3,
        smoothing: bool,
        material: Arc<dyn Material + Sync + Send>,
    ) -> TrianglePrimitive {
        TrianglePrimitive {
            a,
            b,
            c,
            an,
            bn,
            cn,
            mat: Mat3::from_cols(a, b, c).inverse(),
            n,
            d: a.dot(n),
            material,
            smoothing,
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
    fn get_material(&self) -> Arc<dyn Material + Sync + Send> {
        self.material.clone()
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let p0 = self.a;
        let p1 = self.b;
        let p2 = self.c;

        let normal = self.n;

        if ray.direction().dot(normal).abs() < EPSILON {
            return vec![];
        }

        let d = self.d;

        let t = (d - normal.dot(ray.position())) / normal.dot(ray.direction());

        let p = ray.position() + t * ray.direction();

        let v0 = (p - p0).cross(p1 - p0).dot(normal);
        let v1 = (p - p1).cross(p2 - p1).dot(normal);
        let v2 = (p - p2).cross(p0 - p2).dot(normal);

        let t0 = v0 >= -EPSILON;
        let t1 = v1 >= -EPSILON;
        let t2 = v2 >= -EPSILON;

        if t0 == t1 && t1 == t2 {
            vec![if self.smoothing {
                let res = self.mat.mul_vec3(p);

                let smoothed_normal = self.an * res.x + self.bn * res.y + self.cn * res.z;

                Hit::new(p, smoothed_normal, t, Box::new(self), t0)
            } else {
                Hit::new(p, normal, t, Box::new(self), t0)
            }]
        } else {
            vec![]
        }
    }

    fn clone_dyn(&self) -> Box<dyn Primitive + Sync + Send> {
        Box::new(self.clone())
    }
}
