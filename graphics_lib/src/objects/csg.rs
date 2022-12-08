use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use crate::scene::Scene;
use glam::{Affine3A, Vec3};
use std::fmt::Debug;

#[derive(Debug)]
pub enum CSGType {
    Union,
    Intersection,
    Subtract,
}

#[derive(Debug)]
pub struct CSG {
    csg_type: CSGType,
    left: Box<dyn Object + Sync + Send>,
    right: Box<dyn Object + Sync + Send>,
    csg_index: usize,
}

impl CSG {
    pub fn new(
        csg_type: CSGType,
        left: Box<dyn Object + Sync + Send>,
        right: Box<dyn Object + Sync + Send>,
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
    // Apply transform to each child
    fn apply_transform(&mut self, t: &Affine3A) {
        self.left.apply_transform(t);
        self.right.apply_transform(t);
    }

    // Works out which child object the hit belonged to.
    //      Uses the CSG index to index into the tree
    fn get_material(&self, hit: &Hit) -> usize {
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

    // Populates the SCG index tree.
    // eg.     1
    //        / \
    //       2   3
    //          / \
    //         6   7
    // This is used later to identify what side of the tree each hit belongs to
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

    // Filters and modifies the hits to remove the CSG hits that dont exist
    fn filter_hits<'a>(&self, hits: Vec<Hit>, index: usize) -> Vec<Hit> {
        // Applies the filtering of the children first
        let hits = self.left.filter_hits(hits, index);
        let hits = self.right.filter_hits(hits, index);

        // Does the ray start inside either of the objects?
        let mut inside_left = {
            hits.iter()
                .find(|h| {
                    h.get_object_index() == index
                        && is_node_left(self.csg_index, h.get_csg_index()) == Some(true)
                })
                .map_or(false, |h| !h.get_dir())
        };
        let mut inside_right = {
            hits.iter()
                .find(|h| {
                    h.get_object_index() == index
                        && is_node_left(self.csg_index, h.get_csg_index()) == Some(false)
                })
                .map_or(false, |h| !h.get_dir())
        };

        let mut prev_inside_left = inside_left;
        let mut prev_inside_right = inside_right;

        // Iterate through the hits and apply the CSG logic for each case
        hits.into_iter()
            .filter_map(|mut h| {
                if h.get_object_index() != index {
                    Some(h)
                } else if let Some(side) = is_node_left(self.csg_index, h.get_csg_index()) {
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
                            if (h.get_dir() && (inside_left || inside_right))
                                || (!h.get_dir() && !inside_left && !inside_right)
                            {
                                Some(h)
                            } else {
                                None
                            }
                        }
                        CSGType::Intersection => {
                            if (h.get_dir() && inside_left && inside_right)
                                || (!h.get_dir() && (!inside_left || !inside_right))
                            {
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
            })
            .collect()
    }

    // Computes a bounding box for a CSG
    fn get_caustic_bounds(&self) -> (Vec3, Vec3) {
        let left_bound = self.left.get_caustic_bounds();
        let right_bound = self.right.get_caustic_bounds();
        match self.csg_type {
            CSGType::Union => (
                left_bound.0.min(right_bound.0),
                left_bound.1.max(right_bound.1),
            ),
            CSGType::Intersection => (
                left_bound.0.max(right_bound.0),
                left_bound.1.min(right_bound.1),
            ),
            CSGType::Subtract => left_bound,
        }
    }

    // If either child needs a caustic, the CSG needs a caustic
    fn needs_caustic(&self, scene: &Scene) -> bool {
        match self.csg_type {
            CSGType::Subtract => self.left.needs_caustic(scene),
            _ => self.left.needs_caustic(scene) || self.right.needs_caustic(scene),
        }
    }
}

// Helper function for the CSG index tree
// If the child is on the left, returns Some(true), if on the right returns Some(false)
// Otherwise returns None
fn is_node_left(parent: usize, child: usize) -> Option<bool> {
    let mut current = child;
    while current > parent {
        if parent * 2 == current {
            return Some(true);
        }
        if parent * 2 + 1 == current {
            return Some(false);
        }
        current /= 2;
    }
    None
}
