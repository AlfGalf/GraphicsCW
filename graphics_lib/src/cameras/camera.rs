use crate::ray::Ray;
use std::fmt::Debug;

// Trait for camera objects
pub trait Camera: Debug {
    // A camera makes a number of rays for a pixel location in the frame
    // The rays get averaged at render time
    fn rays(&self, x: f32, y: f32) -> Vec<Ray>;
}
