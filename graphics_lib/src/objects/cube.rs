use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::triangle::TrianglePrimitive;
use crate::scene::Scene;
use glam::{DAffine3, DVec3};

#[derive(Debug)]
pub struct Cube {
    material: usize,
    csg_index: usize,
    transform: DAffine3,
}

impl Cube {
    // Cube always starts as cube centered at origin of side length 1
    // Can be transformed to move and resize
    pub fn new(material: usize) -> Self {
        Self {
            material,
            transform: DAffine3::IDENTITY,
            csg_index: 0,
        }
    }

    // Defines the vertices of the triangles making up
    //   a cube of side length 1 at the origin
    fn get_triangles(&self) -> Vec<(DVec3, DVec3, DVec3)> {
        let triangles: Vec<(DVec3, DVec3, DVec3)> = vec![
            (
                DVec3::new(-0.5, -0.5, -0.5),
                DVec3::new(-0.5, 0.5, 0.5),
                DVec3::new(-0.5, -0.5, 0.5),
            ),
            (
                DVec3::new(-0.5, -0.5, -0.5),
                DVec3::new(-0.5, 0.5, -0.5),
                DVec3::new(-0.5, 0.5, 0.5),
            ), // -x side
            (
                DVec3::new(0.5, -0.5, -0.5),
                DVec3::new(0.5, -0.5, 0.5),
                DVec3::new(0.5, 0.5, 0.5),
            ),
            (
                DVec3::new(0.5, -0.5, -0.5),
                DVec3::new(0.5, 0.5, 0.5),
                DVec3::new(0.5, 0.5, -0.5),
            ), // +x side
            (
                DVec3::new(-0.5, -0.5, -0.5),
                DVec3::new(-0.5, -0.5, 0.5),
                DVec3::new(0.5, -0.5, 0.5),
            ),
            (
                DVec3::new(-0.5, -0.5, -0.5),
                DVec3::new(0.5, -0.5, 0.5),
                DVec3::new(0.5, -0.5, -0.5),
            ), // -y side
            (
                DVec3::new(-0.5, 0.5, -0.5),
                DVec3::new(0.5, 0.5, 0.5),
                DVec3::new(-0.5, 0.5, 0.5),
            ),
            (
                DVec3::new(-0.5, 0.5, -0.5),
                DVec3::new(0.5, 0.5, -0.5),
                DVec3::new(0.5, 0.5, 0.5),
            ), // +y side
            (
                DVec3::new(-0.5, -0.5, -0.5),
                DVec3::new(0.5, 0.5, -0.5),
                DVec3::new(-0.5, 0.5, -0.5),
            ),
            (
                DVec3::new(-0.5, -0.5, -0.5),
                DVec3::new(0.5, -0.5, -0.5),
                DVec3::new(0.5, 0.5, -0.5),
            ), // -z side
            (
                DVec3::new(-0.5, -0.5, 0.5),
                DVec3::new(-0.5, 0.5, 0.5),
                DVec3::new(0.5, 0.5, 0.5),
            ),
            (
                DVec3::new(-0.5, -0.5, 0.5),
                DVec3::new(0.5, 0.5, 0.5),
                DVec3::new(0.5, -0.5, 0.5),
            ), // +z side
        ];

        triangles
            .into_iter()
            .map(|(p1, p2, p3)| {
                (
                    // Transforms each triangle
                    self.transform.transform_point3(p1),
                    self.transform.transform_point3(p2),
                    self.transform.transform_point3(p3),
                )
            })
            .collect()
    }
}

impl Object for Cube {
    fn apply_transform(&mut self, t: &DAffine3) {
        self.transform = *t * self.transform
    }

    fn get_material(&self, _: &Hit) -> usize {
        self.material
    }

    fn set_csg_index(&mut self, csg_index: usize) {
        self.csg_index = csg_index;
    }

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>> {
        let triangles: Vec<Box<dyn Primitive + Sync + Send>> = self
            .get_triangles()
            .into_iter()
            .map::<Box<dyn Primitive + Sync + Send>, _>(|(p1, p2, p3)| {
                Box::new(TrianglePrimitive::new(
                    p1,
                    p2,
                    p3,
                    (p3 - p1).cross(p2 - p1).normalize(),
                    DVec3::new(0., 0., 0.),
                    DVec3::new(0., 0., 0.),
                    DVec3::new(0., 0., 0.),
                    false,
                    obj_index,
                    self.csg_index,
                ))
            })
            .collect();
        triangles
    }

    fn filter_hits(&self, hits: Vec<Hit>, _: usize) -> Vec<Hit> {
        hits
    }

    // Finds a bounding box for the object
    fn get_caustic_bounds(&self) -> (DVec3, DVec3) {
        self.get_triangles().into_iter().fold(
            (
                DVec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
                DVec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            ),
            |(c_min, c_max), (p1, p2, p3)| {
                (c_min.min(p1).min(p2).min(p3), c_max.max(p1).max(p2).max(p3))
            },
        )
    }

    fn needs_caustic(&self, scene: &Scene) -> bool {
        scene.material_needs_caustic(self.material)
    }
}
