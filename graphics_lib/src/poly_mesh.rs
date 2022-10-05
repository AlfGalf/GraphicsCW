use crate::ray::Ray;
use crate::transform::Transform;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Triange(pub usize, pub usize, pub usize);

#[derive(Debug)]
pub struct PolyMesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triange>,
    smoothing: bool,
}

impl PolyMesh {
    pub fn from_file(file: BufReader<File>, smooth: bool) -> Result<PolyMesh, &'static str> {
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

                Vertex(
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

                Triange(
                    split_line
                        .next()
                        .ok_or("Missing face (1)")?
                        .parse()
                        .map_err(|_| "Malformed vertex index")?,
                    split_line
                        .next()
                        .ok_or("Missing face (2)")?
                        .parse()
                        .map_err(|_| "Malformed vertex index")?,
                    split_line
                        .next()
                        .ok_or("Missing face (3)")?
                        .parse()
                        .map_err(|_| "Malformed vertex index")?,
                )
            })
        }

        Ok(PolyMesh {
            vertices,
            triangles: faces,
            smoothing: false,
        })
    }

    pub fn apply_transform(&self, tr: &Transform) -> Self {
        Self {
            vertices: self
                .vertices
                .iter()
                .map(|v| v.apply_transform(tr))
                .collect(),
            triangles: self.triangles.clone(),
            smoothing: self.smoothing,
        }
    }

    pub fn intersections(&self, ray: &Ray) -> Option<f32> {
        let mut intersections = self
            .triangles
            .iter()
            .filter_map(|t| {
                let epsilon = 0.000001;

                let p0 = self.vertices.get(t.0).unwrap();
                let p1 = self.vertices.get(t.1).unwrap();
                let p2 = self.vertices.get(t.2).unwrap();

                let edge1 = p0.to(p1);
                let edge2 = p0.to(p2);

                let h = ray.direction.cross(&edge2);
                let a = edge1.dot(&h);

                if a > -epsilon && a < epsilon {
                    return None;
                }

                let f = 1.0 / a;

                let s = p0.to(&ray.position);
                let u = f * s.dot(&h);

                if u < 0.0 || u > 1.0 {
                    return None;
                }

                let q = s.cross(&edge1);
                let v = f * ray.direction.dot(&q);

                if v < 0.0 || u + v > 1.0 {
                    return None;
                }

                let t = f * edge2.dot(&q);

                if t > epsilon {
                    Some(t)
                } else {
                    None
                }
            })
            .collect::<Vec<f32>>();

        intersections.sort_by(|l, r| l.partial_cmp(r).unwrap());

        intersections.first().map(|f| *f)
    }
}
