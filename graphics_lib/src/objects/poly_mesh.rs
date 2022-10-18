use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::triangle::TrianglePrimitive;
use glam::{Affine3A, Vec3};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Triangle { a, b, c }
    }
}

#[derive(Debug)]
pub struct PolyMesh<M: Material> {
    pub triangles: Vec<Triangle>,
    smoothing: bool,
    material: M,
}

impl<M: Material> PolyMesh<M> {
    pub fn from_file(
        file: BufReader<File>,
        material: M,
        smooth: bool,
    ) -> Result<PolyMesh<M>, &'static str> {
        let mut lines = file.lines();

        if let Some(first) = lines.next() {
            if let Ok(str) = first {
                if str != "kcply" {
                    return Err("Wrong file format.");
                }
            } else {
                return Err("Failed to read line.");
            }
        } else {
            return Err("File empty.");
        }

        let num_vertices = {
            let vertices_line = lines
                .next()
                .ok_or("No vertex line present.")?
                .map_err(|_| "Failed to read line.")?;

            if !vertices_line.starts_with("element vertex") {
                return Err("Vertex line malformed (wrong start).");
            }

            let vertex_str = vertices_line
                .split(" ")
                .nth(2)
                .ok_or("Vertex line malformed (no number).")?;

            vertex_str
                .parse::<usize>()
                .map_err(|_| "Vertex number malformed.")?
        };

        let num_faces = {
            let faces_line = lines
                .next()
                .ok_or("No faces line present.")?
                .map_err(|_| "Failed to read line.")?;

            if !faces_line.starts_with("element face ") {
                return Err("Faces line malformed (wrong start).");
            }

            let faces_str = faces_line
                .split(" ")
                .nth(2)
                .ok_or("Faces line malformed (no number).")?;

            faces_str
                .parse::<usize>()
                .map_err(|_| "Faces number malformed.")?
        };

        let mut vertices = Vec::with_capacity(num_vertices as usize);

        for _ in 0..num_vertices {
            vertices.push({
                let line = lines
                    .next()
                    .ok_or("Vertex line missing")?
                    .map_err(|_| "Cannot read vertex line.")?;

                let mut split_line = line.split(" ");

                Vec3::new(
                    split_line
                        .next()
                        .ok_or("Missing vertex (1)")?
                        .parse()
                        .map_err(|_| "Malformed coordinate")?,
                    split_line
                        .next()
                        .ok_or("Missing vertex (2)")?
                        .parse()
                        .map_err(|_| "Malformed coordinate")?,
                    split_line
                        .next()
                        .ok_or("Missing vertex (3)")?
                        .parse()
                        .map_err(|_| "Malformed coordinate")?,
                )
            })
        }

        let mut faces = Vec::with_capacity(num_faces as usize);

        for _ in 0..num_faces {
            faces.push({
                let line = lines
                    .next()
                    .ok_or("Face line missing")?
                    .map_err(|_| "Cannot read face line.")?;

                let mut split_line = line.split(" ");

                if split_line
                    .next()
                    .ok_or("Missing vertex (0)")?
                    .parse::<usize>()
                    .map_err(|e| {
                        println!("{:?} {}", e, line);
                        "Malformed coordinate"
                    })?
                    != 3
                {
                    return Err("Face does not start with 3.");
                }

                let a: &Vec3 = vertices
                    .get(
                        split_line
                            .next()
                            .ok_or("Missing face (1)")?
                            .parse::<usize>()
                            .map_err(|_| "Malformed vertex index")?,
                    )
                    .ok_or("Face uses vertex out of range.")?;
                let b: &Vec3 = vertices
                    .get(
                        split_line
                            .next()
                            .ok_or("Missing face (1)")?
                            .parse::<usize>()
                            .map_err(|_| "Malformed vertex index")?,
                    )
                    .ok_or("Face uses vertex out of range.")?;
                let c: &Vec3 = vertices
                    .get(
                        split_line
                            .next()
                            .ok_or("Missing face (1)")?
                            .parse::<usize>()
                            .map_err(|_| "Malformed vertex index")?,
                    )
                    .ok_or("Face uses vertex out of range.")?;

                Triangle::new(*a, *b, *c)
            })
        }

        Ok(PolyMesh {
            triangles: faces,
            smoothing: smooth,
            material,
        })
    }
}

impl<M: Material + Clone> Object for PolyMesh<M> {
    fn apply_transform(self: &mut PolyMesh<M>, tr: &Affine3A) {
        self.triangles = self
            .triangles
            .iter()
            .map(|t| {
                Triangle::new(
                    tr.transform_point3(t.a),
                    tr.transform_point3(t.b),
                    tr.transform_point3(t.c),
                )
            })
            .collect();
    }

    fn get_material(&self) -> Box<&dyn Material> {
        Box::new(&self.material)
    }

    fn primitives(&self, material_index: usize) -> Vec<Box<dyn Primitive + Sync>> {
        self.triangles
            .iter()
            .map::<Box<dyn Primitive + Sync>, _>(|t| {
                Box::new(TrianglePrimitive::new(t.a, t.b, t.c, material_index))
            })
            .collect()
    }
}
