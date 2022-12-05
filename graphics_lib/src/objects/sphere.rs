use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::sphere::SpherePrimitive;
use crate::scene::Scene;
use glam::{Affine3A, Vec3};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    rad: f32,
    material: usize,
    csg_index: usize,
}

impl Sphere {
    pub fn new(center: Vec3, rad: f32, material: usize) -> Sphere {
        Sphere {
            center,
            rad,
            material,
            csg_index: 0,
        }
    }
}

impl Object for Sphere {
    fn apply_transform(self: &mut Sphere, t: &Affine3A) {
        self.center = t.transform_point3(self.center);
    }

    fn get_material(&self, _: &Hit) -> usize {
        self.material
    }

    fn set_csg_index(&mut self, csg_index: usize) {
        self.csg_index = csg_index
    }

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>> {
        vec![Box::new(SpherePrimitive::new(
            self.center,
            self.rad,
            obj_index,
            self.csg_index,
        ))]
    }

    fn filter_hits(&self, hits: Vec<Hit>, _: usize) -> Vec<Hit> {
        hits
    }

    fn get_caustic_bounds(&self) -> (Vec3, Vec3) {
        (
            self.center - Vec3::new(self.rad, self.rad, self.rad),
            self.center + Vec3::new(self.rad, self.rad, self.rad),
        )
    }

    fn needs_caustic(&self, scene: &Scene) -> bool {
        scene.material_needs_caustic(self.material)
    }
}
