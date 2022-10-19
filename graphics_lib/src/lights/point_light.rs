use crate::color::Color;
use crate::lights::light::Light;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;

const EPSILON: f32 = 0.001;

#[derive(Debug)]
pub struct PointLight {
    position: Vec3,
    color: Color,
}

impl PointLight {
    pub fn new(position: Vec3, color: Color) -> Self {
        Self { position, color }
    }
}

impl Light for PointLight {
    fn get_intensity(&self, point: &Vec3, scene: &Scene) -> Color {
        let distance = point.distance(self.position);
        let ray = Ray::new(self.position, (*point - self.position).normalize());

        if scene
            .intersection(&ray)
            .filter(|r| r.get_distance() > 0. && r.get_distance() < distance - EPSILON)
            .next()
            .is_some()
        {
            Color::new_black()
        } else {
            self.color
        }
    }

    fn get_direction(&self, point: &Vec3) -> Vec3 {
        (self.position - *point).normalize()
    }
}
