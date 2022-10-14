use crate::hit::Hit;
use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::ray::Ray;
use glam::{Affine3A, Vec3};

#[derive(Debug)]
pub struct Sphere<M: Material> {
    center: Vec3,
    rad: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, rad: f32, material: M) -> Sphere<M> {
        Sphere {
            center,
            rad,
            material,
        }
    }
}

impl<M: Material + Clone> Object for Sphere<M> {
    fn intersection(&self, ray: &Ray) -> Option<Hit> {
        // (D.D)*t2+(D.P)*2t+(P.P–R2) =0
        let relative_position: Vec3 = ray.position - self.center;
        let b = 2. * ray.direction.dot(relative_position);
        let c = relative_position.dot(relative_position) - self.rad * self.rad;

        let discriminant: f32 = b * b - 4. * c;
        if discriminant <= 0. {
            None
        } else {
            let sqrt = discriminant.sqrt();
            let t0 = (sqrt - b) / 2.;
            let t1 = (-sqrt - b) / 2.;
            let t = t0.min(t1);

            let pos = ray.position + ray.direction * t;
            Some(Hit::new(pos, (pos - self.center).normalize(), t))
        }
    }

    fn apply_transform(self: &mut Sphere<M>, t: &Affine3A) {
        self.center = t.transform_point3(self.center);
    }

    fn get_material(&self) -> Box<&dyn Material> {
        Box::new(&self.material)
    }
}
