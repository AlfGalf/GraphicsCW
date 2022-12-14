use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::primitives::triangle::TrianglePrimitive;
use crate::scene::Scene;
use glam::{DAffine3, DVec3};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Triangle {
    // Indices into vertex in polymesh array
    an: usize,
    bn: usize,
    cn: usize,
    pub n: DVec3,
}

#[derive(Debug)]
struct Vertex {
    // indices of surrounding triangles
    pub(crate) triangles: Vec<usize>,
    p: DVec3,
    // Normal may be calculated for smoothing
    normal: Option<DVec3>,
}

impl Vertex {
    pub fn apply_transform(&mut self, tr: &DAffine3) {
        self.p = tr.transform_point3(self.p)
    }

    pub fn compute_normal(&mut self, tr: &[Triangle]) {
        // Average of the the triangles it is part of
        self.normal = Some({
            let normal_sum = self
                .triangles
                .iter()
                .fold(DVec3::ZERO, |v, i| v + tr.get(*i).unwrap().n);
            normal_sum / (self.triangles.len() as f64)
        });
    }
}

impl Triangle {
    fn new(vs: &[Vertex], an: usize, bn: usize, cn: usize) -> Self {
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

    fn update_normal(&mut self, vs: &[Vertex]) {
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
    csg_index: usize,
}

impl PolyMesh {
    // Makes a Polymesh from a file
    // Note this can fail!
    pub fn from_file(
        file: BufReader<File>,
        material: usize,
        smooth: bool,
        ord_rev: bool,
    ) -> Result<PolyMesh, String> {
        let mut lines = file.lines();

        if let Some(first) = lines.next() {
            if let Ok(str) = first {
                if str != "kcply" {
                    return Err("Wrong file format.".to_string());
                }
            } else {
                return Err("Failed to read line.".to_string());
            }
        } else {
            return Err("File empty.".to_string());
        }

        let num_vertices = {
            let vertices_line = lines
                .next()
                .ok_or("No vertex line present.")?
                .map_err(|_| "Failed to read line.")?;

            if !vertices_line.starts_with("element vertex") {
                return Err("Vertex line malformed (wrong start).".to_string());
            }

            let vertex_str = vertices_line
                .split(' ')
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
                return Err("Faces line malformed (wrong start).".to_string());
            }

            let faces_str = faces_line
                .split(' ')
                .nth(2)
                .ok_or("Faces line malformed (no number).")?;

            faces_str
                .parse::<usize>()
                .map_err(|_| "Faces number malformed.")?
        };

        let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices as usize);

        for i in 0..num_vertices {
            let line = lines
                .next()
                .ok_or("Vertex line missing")?
                .map_err(|_| "Cannot read vertex line.")?;

            let mut split_line = line.split(' ');
            vertices.push(Vertex {
                p: DVec3::new(
                    split_line
                        .next()
                        .ok_or("Missing vertex (1)")?
                        .parse()
                        .map_err(|e: _| format!("Malformed coordinate {}.1", i))?,
                    split_line
                        .next()
                        .ok_or("Missing vertex (2)")?
                        .parse()
                        .map_err(|e: _| format!("Malformed coordinate {}.2", i))?,
                    split_line
                        .next()
                        .ok_or("Missing vertex (3)")?
                        .parse()
                        .map_err(|e: _| format!("Malformed coordinate {}.3", i))?,
                ),
                triangles: vec![],
                normal: None,
            });
        }

        let mut faces = Vec::with_capacity(num_faces as usize);

        // Populates faces vector
        for i in 0..num_faces {
            faces.push({
                let line = lines
                    .next()
                    .ok_or("Face line missing")?
                    .map_err(|_| "Cannot read face line.")?;

                let mut split_line = line.split(' ');

                if split_line
                    .next()
                    .ok_or("Missing vertex (0)")?
                    .parse::<usize>()
                    .map_err(|_| "Malformed coordinate")?
                    != 3
                {
                    return Err("Face does not start with 3.".to_string());
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

                let (bn, cn) = if ord_rev { (cn, bn) } else { (bn, cn) };

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
            csg_index: 0,
        };

        // After faces vector populated, then the triangles normals can be calculated
        for v in pm.vertices.iter_mut() {
            v.compute_normal(&pm.triangles);
        }

        Ok(pm)
    }
}

impl Object for PolyMesh {
    fn apply_transform(self: &mut PolyMesh, tr: &DAffine3) {
        // After transforming normals must be recomputed
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

    fn get_material(&self, _: &Hit) -> usize {
        self.material
    }

    fn set_csg_index(&mut self, csg_index: usize) {
        self.csg_index = csg_index;
    }

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>> {
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
                    obj_index,
                    self.csg_index,
                ))
            })
            .collect()
    }

    fn filter_hits(&self, hits: Vec<Hit>, _: usize) -> Vec<Hit> {
        hits
    }

    fn get_caustic_bounds(&self) -> (DVec3, DVec3) {
        // Find the upper and lower bounds of the vertices of the triangles
        self.triangles.iter().fold(
            (
                DVec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
                DVec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            ),
            |(c_min, c_max), t| {
                (
                    c_min
                        .min(self.vertices[t.an].p)
                        .min(self.vertices[t.bn].p)
                        .min(self.vertices[t.cn].p),
                    c_max
                        .max(self.vertices[t.an].p)
                        .max(self.vertices[t.bn].p)
                        .max(self.vertices[t.cn].p),
                )
            },
        )
    }

    fn needs_caustic(&self, scene: &Scene) -> bool {
        scene.material_needs_caustic(self.material)
    }
}
