use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use glam::Affine3A;
use std::fmt::Debug;
use std::net::Shutdown::Read;

#[derive(Debug)]
pub enum CSGType {
    Union,
    Intersection,
    Subtract,
}

#[derive(Debug)]
pub struct CSG {
    csg_type: CSGType,
    left: Box<dyn Object + Sync>,
    right: Box<dyn Object + Sync>,
    csg_index: usize,
}

impl CSG {
    pub fn new(
        csg_type: CSGType,
        left: Box<dyn Object + Sync>,
        right: Box<dyn Object + Sync>,
    ) -> Self {
        Self {
            csg_type,
            left,
            right,
            csg_index: 0,
        }
    }
}

impl Object for CSG {
    fn apply_transform(&mut self, t: &Affine3A) {
        self.left.apply_transform(t);
        self.right.apply_transform(t);
    }

    fn get_material(&self, hit: &Hit) -> usize {
        match self.csg_type {
            CSGType::Subtract => self.left.get_material(hit),
            _ => {
                if let Some(is_left) = is_node_left(self.csg_index, hit.get_csg_index()) {
                    if is_left {
                        self.left.get_material(hit)
                    } else {
                        self.right.get_material(hit)
                    }
                } else {
                    self.left.get_material(hit)
                }
            }
        }
    }

    fn set_csg_index(&mut self, csg_index: usize) {
        self.csg_index = csg_index;
        self.left.set_csg_index(csg_index * 2);
        self.right.set_csg_index(csg_index * 2 + 1);
    }

    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>> {
        let mut a = self.left.primitives(obj_index);
        a.append(&mut self.right.primitives(obj_index));
        a
    }

    // TODO: Sort out the materials
    fn filter_hits<'a>(&self, hits: Vec<Hit>, index: usize) -> Vec<Hit> {
        let hits = self.left.filter_hits(hits, index);
        let hits = self.right.filter_hits(hits, index);

        let mut inside_left = {
            hits.iter()
                .filter(|h| {
                    h.get_object_index() == index
                        && is_node_left(self.csg_index, h.get_csg_index()) == Some(true)
                })
                .next()
                .map_or(false, |h| !h.get_dir())
        };
        let mut inside_right = {
            hits.iter()
                .filter(|h| {
                    h.get_object_index() == index
                        && is_node_left(self.csg_index, h.get_csg_index()) == Some(false)
                })
                .next()
                .map_or(false, |h| !h.get_dir())
        };

        let mut prev_inside_left = inside_left;
        let mut prev_inside_right = inside_right;

        let output = hits
            .into_iter()
            .filter_map(|mut h| {
                if !(h.get_object_index() == index) {
                    Some(h)
                } else {
                    if let Some(side) = is_node_left(self.csg_index, h.get_csg_index()) {
                        prev_inside_left = inside_left;
                        prev_inside_right = inside_right;
                        if side {
                            // Left case
                            inside_left = h.get_dir();
                        } else {
                            // Right case
                            inside_right = h.get_dir();
                        };

                        match self.csg_type {
                            CSGType::Union => {
                                if h.get_dir() && (inside_left || inside_right) {
                                    Some(h)
                                } else if !h.get_dir() && !inside_left && !inside_right {
                                    Some(h)
                                } else {
                                    None
                                }
                            }
                            CSGType::Intersection => {
                                if h.get_dir() && inside_left && inside_right {
                                    Some(h)
                                } else if !h.get_dir() && (!inside_left || !inside_right) {
                                    Some(h)
                                } else {
                                    None
                                }
                            }
                            CSGType::Subtract => {
                                if inside_left
                                    && !prev_inside_left
                                    && !inside_right
                                    && !prev_inside_right
                                {
                                    // Case, just entered left, not in right
                                    Some(h)
                                    // None
                                } else if !inside_left
                                    && prev_inside_left
                                    && !inside_right
                                    && !prev_inside_right
                                {
                                    // Case, just exited left, not in right
                                    Some(h)
                                    // None
                                } else if inside_left
                                    && prev_inside_left
                                    && inside_right
                                    && !prev_inside_right
                                {
                                    // Case, just entered right, inside left
                                    h.flip();
                                    Some(h)
                                    // None
                                } else if inside_left
                                    && prev_inside_left
                                    && !inside_right
                                    && prev_inside_right
                                {
                                    // Case, just exited right, inside left
                                    h.flip();
                                    Some(h)
                                    // None
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        Some(h)
                    }
                }
            })
            .collect();
        output
    }
}

fn is_node_left(parent: usize, child: usize) -> Option<bool> {
    let mut current = child;
    while current > parent {
        if parent * 2 == current {
            return Some(true);
        }
        if parent * 2 + 1 == current {
            return Some(false);
        }
        current = current / 2;
    }
    None
}
