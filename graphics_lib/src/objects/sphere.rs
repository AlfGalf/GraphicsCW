use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::sphere::SpherePrimitive;
use crate::scene::Scene;
use glam::{DAffine3, DVec3};

#[derive(Debug)]
pub struct Sphere {
    center: DVec3,
    rad: f64,
    material: usize,
    csg_index: usize,
}

impl Sphere {
    pub fn new(center: DVec3, rad: f64, material: usize) -> Sphere {
        Sphere {
            center,
            rad,
            material,
            csg_index: 0,
        }
    }
}

impl Object for Sphere {
    // Spheres do *not* fully support general transforms, and instead approximate them
    // Use Quadratic curves for full transform support
    fn apply_transform(self: &mut Sphere, t: &DAffine3) {
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

    fn get_caustic_bounds(&self) -> (DVec3, DVec3) {
        (
            self.center - DVec3::new(self.rad, self.rad, self.rad),
            self.center + DVec3::new(self.rad, self.rad, self.rad),
        )
    }

    fn needs_caustic(&self, scene: &Scene) -> bool {
        scene.material_needs_caustic(self.material)
    }
}
