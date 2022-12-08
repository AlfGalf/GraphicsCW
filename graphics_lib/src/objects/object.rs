use crate::hit::Hit;
use crate::primitives::primitive::Primitive;
use crate::scene::Scene;
use glam::{Affine3A, Vec3};
use std::fmt::Debug;

// This represents an object in the scene
pub trait Object: Debug {
    // Transforms the object
    fn apply_transform(&mut self, t: &Affine3A);

    // Get the index of the objects material from the scene
    fn get_material(&self, hit: &Hit) -> usize;

    // Sets the CSG index of an object,
    // Used to work out which object in a CSG tree an object belongs to
    fn set_csg_index(&mut self, csg_index: usize);

    // Get a list of primitives from the object
    fn primitives(&self, obj_index: usize) -> Vec<Box<dyn Primitive + Sync + Send>>;

    // Each time intersections are calculated, each object gets the chance to modify the hits
    // Only used by the CSG
    fn filter_hits(&self, hits: Vec<Hit>, index: usize) -> Vec<Hit>;

    // Gets a bounding box for the object to fire rays within
    fn get_caustic_bounds(&self) -> (Vec3, Vec3);

    // Works out if an object needs caustics or not
    fn needs_caustic(&self, scene: &Scene) -> bool;
}
