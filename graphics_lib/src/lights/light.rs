use crate::color::Color;
use crate::scene::Scene;
use glam::Vec3;
use std::fmt::Debug;

pub trait Light: Debug {
    fn get_intensity(&self, point: &Vec3, scene: &Scene) -> Color;
    fn get_direction(&self, point: &Vec3) -> Vec3;
}
