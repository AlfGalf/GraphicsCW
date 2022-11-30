use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::triangle::TrianglePrimitive;
use glam::{Affine3A, Vec3};

#[derive(Debug)]
pub struct Cube {
    material: usize,
    csg_index: usize,
    transform: Affine3A,
}

impl Cube {
    pub fn new(material: usize) -> Self {
        Self {
            material,
            transform: Affine3A::IDENTITY,
            csg_index: 0,
        }
    }
}

impl Object for Cube {
    fn apply_transform(&mut self, t: &Affine3A) {
        self.transform = self.transform * *t
    }

    fn get_material(&self, hit: &Hit) -> usize {
        self.material
    }

    fn set_csg_index(&mut self, csg_index: usize) {
        self.csg_index = csg_index;
    }

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>> {
        let triangles: Vec<(Vec3, Vec3, Vec3)> = vec![
            (
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(-0.5, -0.5, 0.5),
            ),
            (
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ), // -x side
            (
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
            ),
            (
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, -0.5),
            ), // +x side
            (
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
            ),
            (
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, -0.5),
            ), // -y side
            (
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ),
            (
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, 0.5),
            ), // +y side
            (
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
            ),
            (
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
            ), // -z side
            (
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
            ),
            (
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
            ), // +z side
        ];

        let triangles: Vec<Box<dyn Primitive + Sync + Send>> = triangles
            .into_iter()
            .map::<Box<dyn Primitive + Sync + Send>, _>(|(p1, p2, p3)| {
                let p1 = self.transform.transform_point3(p1);
                let p2 = self.transform.transform_point3(p2);
                let p3 = self.transform.transform_point3(p3);
                Box::new(TrianglePrimitive::new(
                    p1,
                    p2,
                    p3,
                    (p3 - p1).cross(p2 - p1).normalize(),
                    Vec3::new(0., 0., 0.),
                    Vec3::new(0., 0., 0.),
                    Vec3::new(0., 0., 0.),
                    false,
                    self.material,
                    obj_index,
                    self.csg_index,
                ))
            })
            .collect();
        triangles
    }

    fn filter_hits(&self, hits: Vec<Hit>, index: usize) -> Vec<Hit> {
        hits
    }
}
