use crate::color::Color;
use crate::ray::Ray;
use crate::scene::{Light, Scene};
use glam::Vec3;

const EPSILON: f32 = 0.001;

#[derive(Debug)]
pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Color) -> DirectionalLight {
        DirectionalLight {
            direction: direction.normalize(),
            color,
        }
    }
}

impl Light for DirectionalLight {
    fn get_intensity(&self, point: &Vec3, scene: &Scene) -> Color {
        let ray = Ray::new(*point, self.direction);

        if scene
            .objects
            .iter()
            .filter_map(|o| {
                if o.intersection(&ray)?.get_distance() < -EPSILON {
                    Some(())
                } else {
                    None
                }
            })
            .next()
            .is_some()
        {
            Color::new_black()
        } else {
            self.color
        }
    }

    fn get_direction(&self, _: &Vec3) -> Vec3 {
        self.direction
    }
}
