use crate::hit::Hit;
use crate::materials::material::Material;
use crate::primitives::primitive::Primitive;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use glam::Vec3;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SpherePrimitive {
    center: Vec3,
    rad: f32,
    material: Arc<dyn Material + Sync + Send>,
    node_index: usize,
}

impl SpherePrimitive {
    pub(crate) fn new(center: Vec3, rad: f32, material: Arc<dyn Material + Sync + Send>) -> Self {
        Self {
            center,
            rad,
            material,
            node_index: 0,
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
        let half = Vec3::new(self.rad, self.rad, self.rad);
        AABB::with_bounds(self.center - half, self.center + half)
    }
}

impl Primitive for SpherePrimitive {
    fn get_material(&self) -> Arc<dyn Material + Sync + Send> {
        self.material.clone()
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let relative_position: Vec3 = ray.position() - self.center;
        let b = 2. * ray.direction().dot(relative_position);
        let c = relative_position.dot(relative_position) - self.rad * self.rad;

        let discriminant: f32 = b * b - 4. * c;

        if discriminant <= 0. {
            Vec::new()
        } else {
            let sqrt = discriminant.sqrt();
            let t0 = (sqrt - b) / 2.;
            let t1 = (-sqrt - b) / 2.;

            let t_first = t0.min(t1);
            let pos1 = ray.position() + ray.direction() * t_first;
            let t_second = t0.max(t1);
            let pos2 = ray.position() + ray.direction() * t_second;

            // TODO: Make this only intersect if going through normal side
            // TODO: May need to rejig this for rarefaction, light coming in other side

            vec![
                Hit::new(
                    pos1,
                    (pos1 - self.center).normalize(),
                    t_first,
                    Box::new(self),
                    true,
                ),
                Hit::new(
                    pos2,
                    (pos2 - self.center).normalize(),
                    t_second,
                    Box::new(self),
                    false,
                ),
            ]
        }
    }

    fn clone_dyn(&self) -> Box<dyn Primitive + Sync + Send> {
        Box::new(self.clone())
    }
}
