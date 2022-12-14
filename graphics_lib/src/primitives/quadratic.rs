use crate::constants::{EPSILON, SCENE_BOUNDS};
use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::{DMat4, DVec3, Vec3};
use std::fmt::Debug;

#[derive(Debug)]
pub struct QuadraticPrimitive {
    values: [f64; 10],
    csg_index: usize,
    object_index: usize,
    bh_index: usize,
}

impl QuadraticPrimitive {
    pub fn new(mat: DMat4, csg_index: usize, object_index: usize) -> Self {
        let mat_vals = mat.to_cols_array_2d();
        Self {
            values: [
                // Get values from matrix
                mat_vals[0][0],
                mat_vals[0][1],
                mat_vals[0][2],
                mat_vals[0][3],
                mat_vals[1][1],
                mat_vals[1][2],
                mat_vals[1][3],
                mat_vals[2][2],
                mat_vals[2][3],
                mat_vals[3][3],
            ],
            csg_index,
            object_index,
            bh_index: 0,
        }
    }
}

impl BHShape for QuadraticPrimitive {
    fn set_bh_node_index(&mut self, i: usize) {
        self.bh_index = i
    }

    fn bh_node_index(&self) -> usize {
        self.bh_index
    }
}

// Cannot easily bound a quadratic
impl Bounded for QuadraticPrimitive {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(
            Vec3::new(-SCENE_BOUNDS, -SCENE_BOUNDS, -SCENE_BOUNDS),
            Vec3::new(SCENE_BOUNDS, SCENE_BOUNDS, SCENE_BOUNDS),
        )
    }
}

impl Primitive for QuadraticPrimitive {
    fn get_object(&self) -> usize {
        self.object_index
    }

    fn get_csg_index(&self) -> usize {
        self.csg_index
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let values = self.values;

        // This is the equations from the computer graphics slides
        let a = values[0] * ray.direction().x.powi(2)
            + 2. * values[1] * ray.direction().x * ray.direction().y
            + 2. * values[2] * ray.direction().x * ray.direction().z
            + values[4] * ray.direction().y.powi(2)
            + 2. * values[5] * ray.direction().y * ray.direction().z
            + values[7] * ray.direction().z.powi(2);

        let b = 2.
            * (values[0] * ray.position().x * ray.direction().x
                + values[1]
                    * (ray.position().x * ray.direction().y
                        + ray.direction().x * ray.position().y)
                + values[2]
                    * (ray.position().y * ray.direction().z
                        + ray.direction().z * ray.position().y)
                + values[3] * ray.direction().x
                + values[4] * ray.position().y * ray.direction().y
                + values[5]
                    * (ray.position().y * ray.direction().z
                        + ray.direction().y * ray.position().z)
                + values[6] * ray.direction().y
                + values[7] * ray.position().z * ray.direction().z
                + values[8] * ray.direction().z);

        let c = values[0] * ray.position().x.powi(2)
            + 2. * values[1] * ray.position().x * ray.position().y
            + 2. * values[2] * ray.position().x * ray.position().z
            + 2. * values[3] * ray.position().x
            + values[4] * ray.position().y.powi(2)
            + 2. * values[5] * ray.position().y * ray.position().z
            + 2. * values[6] * ray.position().y
            + values[7] * ray.position().z.powi(2)
            + 2. * values[8] * ray.position().z
            + values[9];

        let discriminant = b.powi(2) - 4. * a * c;

        // If discriminant less than 1 (or sufficiently close) no hits
        // Or if a if sufficiently small, as it will be used as a denominator
        // so hit would be very far away
        if discriminant < EPSILON || a.abs() < EPSILON {
            vec![]
        } else {
            let root = discriminant.sqrt();
            let v0 = (-b - root) / (2. * a);
            let v1 = (-b + root) / (2. * a);
            let t0 = v0.min(v1);
            let t1 = v0.max(v1);
            let p0 = ray.position() + t0 * ray.direction();
            let p1 = ray.position() + t1 * ray.direction();
            vec![
                Hit::new(
                    p0,
                    DVec3::new(
                        values[0] * p0.x + values[1] * p0.y + values[2] * p0.z + values[3],
                        values[1] * p0.x + values[4] * p0.y + values[5] * p0.z + values[6],
                        values[2] * p0.x + values[5] * p0.y + values[7] * p0.z + values[8],
                    )
                    .normalize(),
                    t0,
                    true,
                    self.object_index,
                    self.csg_index,
                ),
                Hit::new(
                    p1,
                    DVec3::new(
                        values[0] * p1.x + values[1] * p1.y + values[2] * p1.z + values[3],
                        values[1] * p1.x + values[4] * p1.y + values[5] * p1.z + values[6],
                        values[2] * p1.x + values[5] * p1.y + values[7] * p1.z + values[8],
                    )
                    .normalize(),
                    t1,
                    false,
                    self.object_index,
                    self.csg_index,
                ),
            ]
        }
    }
}
