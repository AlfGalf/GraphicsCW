use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::triangle::TrianglePrimitive;
use glam::{Affine3A, Vec3};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Triangle {
    an: usize,
    bn: usize,
    cn: usize,
    pub n: Vec3,
}

#[derive(Debug)]
struct Vertex {
    pub(crate) triangles: Vec<usize>,
    p: Vec3,
    normal: Option<Vec3>,
}

impl Vertex {
    pub fn apply_transform(&mut self, tr: &Affine3A) {
        self.p = tr.transform_point3(self.p)
    }

    pub fn compute_normal(&mut self, tr: &Vec<Triangle>) {
        self.normal = Some({
            let normal_sum = self
                .triangles
                .iter()
                .fold(Vec3::ZERO, |v, i| v + tr.get(*i).unwrap().n);
            normal_sum / (self.triangles.len() as f32)
        });
    }
}

impl Triangle {
    fn new(vs: &Vec<Vertex>, an: usize, bn: usize, cn: usize) -> Self {
        let av = vs.get(an).unwrap().p;
        let bv = vs.get(bn).unwrap().p;
        let cv = vs.get(cn).unwrap().p;
        let normal = (cv - av).cross(bv - av).normalize();
        Triangle {
            an,
            bn,
            cn,
            n: normal,
        }
    }

    fn update_normal(&mut self, vs: &Vec<Vertex>) {
        let av = vs.get(self.an).unwrap().p;
        let bv = vs.get(self.bn).unwrap().p;
        let cv = vs.get(self.cn).unwrap().p;
        let normal = (cv - av).cross(bv - av).normalize();
        self.n = normal;
    }
}

#[derive(Debug)]
pub struct PolyMesh {
    pub triangles: Vec<Triangle>,
    vertices: Vec<Vertex>,
    smoothing: bool,
    material: usize,
}

impl PolyMesh {
    pub fn from_file(
        file: BufReader<File>,
        material: usize,
        smooth: bool,
    ) -> Result<PolyMesh, &'static str> {
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

        let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices as usize);

        for _ in 0..num_vertices {
            let line = lines
                .next()
                .ok_or("Vertex line missing")?
                .map_err(|_| "Cannot read vertex line.")?;

            let mut split_line = line.split(" ");
            vertices.push(Vertex {
                p: Vec3::new(
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
                ),
                triangles: vec![],
                normal: None,
            });
        }

        let mut faces = Vec::with_capacity(num_faces as usize);

        for i in 0..num_faces {
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

                let an: usize = split_line
                    .next()
                    .ok_or("Missing face (1)")?
                    .parse::<usize>()
                    .map_err(|_| "Malformed vertex index")?;
                let bn: usize = split_line
                    .next()
                    .ok_or("Missing face (2)")?
                    .parse::<usize>()
                    .map_err(|_| "Malformed vertex index")?;
                let cn: usize = split_line
                    .next()
                    .ok_or("Missing face (3)")?
                    .parse::<usize>()
                    .map_err(|_| "Malformed vertex index")?;

                vertices.get_mut(an).unwrap().triangles.push(i);
                vertices.get_mut(bn).unwrap().triangles.push(i);
                vertices.get_mut(cn).unwrap().triangles.push(i);

                Triangle::new(&vertices, an, bn, cn)
            });
        }

        let mut pm = PolyMesh {
            triangles: faces,
            smoothing: smooth,
            vertices,
            material,
        };

        for v in pm.vertices.iter_mut() {
            v.compute_normal(&pm.triangles);
        }

        Ok(pm)
    }
}

impl Object for PolyMesh {
    fn apply_transform(self: &mut PolyMesh, tr: &Affine3A) {
        for v in self.vertices.iter_mut() {
            v.apply_transform(tr);
        }
        for f in self.triangles.iter_mut() {
            f.update_normal(&self.vertices);
        }
        for v in self.vertices.iter_mut() {
            v.compute_normal(&self.triangles);
        }
    }

    fn get_material(&self) -> usize {
        self.material
    }

    fn primitives(&self) -> Vec<Box<dyn Primitive + Sync + Send>> {
        self.triangles
            .iter()
            .map::<Box<dyn Primitive + Sync + Send>, _>(|t| {
                let va = self.vertices.get(t.an).unwrap();
                let vb = self.vertices.get(t.bn).unwrap();
                let vc = self.vertices.get(t.cn).unwrap();

                Box::new(TrianglePrimitive::new(
                    va.p,
                    vb.p,
                    vc.p,
                    t.n,
                    va.normal.unwrap(),
                    vb.normal.unwrap(),
                    vc.normal.unwrap(),
                    self.smoothing,
                    self.material,
                ))
            })
            .collect()
    }
}
