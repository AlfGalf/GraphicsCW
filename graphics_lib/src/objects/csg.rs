use crate::hit::Hit;
use crate::objects::object::Object;
use crate::primitives::primitive::Primitive;
use glam::Affine3A;
use std::fmt::Debug;

#[derive(Debug)]
pub enum CSGType {
    Union,
}

#[derive(Debug)]
pub struct CSG {
    csg_type: CSGType,
    left: Box<dyn Object + Sync>,
    right: Box<dyn Object + Sync>,
    material: usize,
    csg_index: usize,
}

impl CSG {
    pub fn new(
        csg_type: CSGType,
        left: Box<dyn Object + Sync>,
        right: Box<dyn Object + Sync>,
        material: usize,
    ) -> Self {
        Self {
            csg_type,
            left,
            right,
            material,
            csg_index: 0,
        }
    }
}

impl Object for CSG {
    fn apply_transform(&mut self, t: &Affine3A) {
        self.left.apply_transform(t);
        self.right.apply_transform(t);
    }

    fn get_material(&self) -> usize {
        self.material
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

    fn filter_hits<'a>(&self, hits: Vec<Hit>, index: usize) -> Vec<Hit> {
        match self.csg_type {
            CSGType::Union => {
                let output = hits
                    .into_iter()
                    .filter(|h| h.get_object_index() == index || { true })
                    .collect();
                output
            }
        }
    }
}
