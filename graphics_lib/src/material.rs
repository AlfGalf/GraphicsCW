use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;
use std::fmt::Debug;

pub trait Material: Debug {
    fn compute_once(&self, ray: &Ray, hit: &Hit) -> Color;
}
