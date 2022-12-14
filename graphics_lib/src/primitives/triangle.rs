use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::{DMat3, DVec3, Vec3};

const EPSILON: f64 = 1E-5;

#[derive(Clone, Debug)]
pub struct TrianglePrimitive {
    a: DVec3,
    b: DVec3,
    c: DVec3,
    n: DVec3,
    an: DVec3,
    bn: DVec3,
    cn: DVec3,
    mat: DMat3,
    d: f64,
    smoothing: bool,
    node_index: usize,
    obj_index: usize,
    csg_index: usize,
}

impl TrianglePrimitive {
    pub fn new(
        a: DVec3,
        b: DVec3,
        c: DVec3,
        n: DVec3,
        an: DVec3,
        bn: DVec3,
        cn: DVec3,
        smoothing: bool,
        obj_index: usize,
        csg_index: usize,
    ) -> TrianglePrimitive {
        TrianglePrimitive {
            a,
            b,
            c,
            an,
            bn,
            cn,
            mat: DMat3::from_cols(a, b, c).inverse(),
            n,
            d: a.dot(n),
            smoothing,
            node_index: 0,
            obj_index,
            csg_index,
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
            self.a.min(self.b).min(self.c).as_vec3(),
            self.a.max(self.b).max(self.c).as_vec3(),
        )
    }
}

impl Primitive for TrianglePrimitive {
    fn get_object(&self) -> usize {
        self.obj_index
    }

    fn get_csg_index(&self) -> usize {
        self.csg_index
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

        if t0 && t1 && t2 {
            vec![if self.smoothing {
                let res = self.mat.mul_vec3(p);

                let smoothed_normal = self.an * res.x + self.bn * res.y + self.cn * res.z;

                Hit::new(
                    p,
                    smoothed_normal,
                    t,
                    normal.dot(ray.direction()) < 0.,
                    self.obj_index,
                    self.csg_index,
                )
            } else {
                Hit::new(
                    p,
                    normal,
                    t,
                    normal.dot(ray.direction()) < 0.,
                    self.obj_index,
                    self.csg_index,
                )
            }]
        } else {
            vec![]
        }
    }
}
