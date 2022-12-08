use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::quadratic::QuadraticPrimitive;
use crate::scene::Scene;
use glam::{Affine3A, Mat4, Vec3};

#[derive(Debug)]
pub struct Quadratic {
    mat: Mat4,
    material: usize,
    csg_index: usize,
}

impl Quadratic {
    pub fn new(vals: [f32; 10], material: usize) -> Self {
        Self {
            // Uses a Mat4 internally to make transforms easier
            mat: Mat4::from_cols_array(&[
                vals[0], vals[1], vals[2], vals[3], //
                vals[1], vals[4], vals[5], vals[6], //
                vals[2], vals[5], vals[7], vals[8], //
                vals[3], vals[6], vals[8], vals[9],
            ]),
            material,
            csg_index: 0,
        }
    }
}

impl Object for Quadratic {
    fn apply_transform(&mut self, t: &Affine3A) {
        self.mat = Mat4::from(*t).transpose() * self.mat * Mat4::from(*t)
    }

    fn get_material(&self, _: &Hit) -> usize {
        self.material
    }

    fn set_csg_index(&mut self, csg_index: usize) {
        self.csg_index = csg_index
    }

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>> {
        vec![Box::new(QuadraticPrimitive::new(
            self.mat,
            self.csg_index,
            obj_index,
        ))]
    }

    fn filter_hits(&self, hits: Vec<Hit>, _: usize) -> Vec<Hit> {
        hits
    }

    // Cannot easily bound a quadratic as may be unbounded
    // Bust be wrapped in an Intersection CSG to by used for caustics
    fn get_caustic_bounds(&self) -> (Vec3, Vec3) {
        (
            Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
            Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        )
    }

    fn needs_caustic(&self, scene: &Scene) -> bool {
        scene.material_needs_caustic(self.material)
    }
}
