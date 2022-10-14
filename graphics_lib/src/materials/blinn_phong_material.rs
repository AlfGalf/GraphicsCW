use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::fmt::Debug;

const CONTROL_FACTOR: usize = 20;
const DEFAULT_AMBIENT: f32 = 0.2;

#[derive(Debug, Clone)]
pub struct BlinnPhongMaterial {
    ambient: Color,
    diffuse: Color,
    specular: Color,
    power: f32,
}

impl BlinnPhongMaterial {
    pub fn new_from_color(color: Color, shininess: f32) -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient: color * DEFAULT_AMBIENT,
            diffuse: color * (1. - shininess),
            specular: color * shininess,
            power: 1.0,
        }
    }

    pub fn new_from_colors(ambient: Color, diffuse: Color, specular: Color) -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient,
            diffuse,
            specular,
            power: 1.0,
        }
    }

    pub fn new() -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient: Color::new(0.3, 0.3, 0.3),
            diffuse: Color::new(0.5, 0.5, 0.5),
            specular: Color::new(0.1, 0.1, 0.1),
            power: 1.0,
        }
    }
}

impl Material for BlinnPhongMaterial {
    fn compute_once(&self, _: &Ray, _: &Hit, ambient: Color) -> Color {
        Color::new(
            self.ambient.red() * ambient.red(),
            self.ambient.green() * ambient.green(),
            self.ambient.blue() * ambient.blue(),
        )
    }

    fn compute_per_light(&self, viewer: &Ray, hit: &Hit, dir: &Vec3, light: Color) -> Color {
        let diffuse = self.diffuse * hit.normal.dot(-*dir).max(0.);
        let reflection_dir: Vec3 = *dir - 2. * (dir.dot(hit.normal)) * hit.normal;
        let reflection_dir = reflection_dir.normalize();
        let specular = self.specular
            * (reflection_dir
                .dot(-viewer.direction)
                .powi(CONTROL_FACTOR as i32))
            .max(0.);

        light.scale(&(diffuse + specular))
    }
}
