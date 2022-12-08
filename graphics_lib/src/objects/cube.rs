use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::triangle::TrianglePrimitive;
use crate::scene::Scene;
use glam::{Affine3A, Vec3};

#[derive(Debug)]
pub struct Cube {
    material: usize,
    csg_index: usize,
    transform: Affine3A,
}

impl Cube {
    // Cube always starts as cube centered at origin of side length 1
    // Can be transformed to move and resize
    pub fn new(material: usize) -> Self {
        Self {
            material,
            transform: Affine3A::IDENTITY,
            csg_index: 0,
        }
    }

    // Defines the vertices of the triangles making up
    //   a cube of side length 1 at the origin
    fn get_triangles(&self) -> Vec<(Vec3, Vec3, Vec3)> {
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
    fn apply_transform(&mut self, t: &Affine3A) {
        self.transform = self.transform * *t
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
                    Vec3::new(0., 0., 0.),
                    Vec3::new(0., 0., 0.),
                    Vec3::new(0., 0., 0.),
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
    fn get_caustic_bounds(&self) -> (Vec3, Vec3) {
        self.get_triangles().into_iter().fold(
            (
                Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
                Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
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
