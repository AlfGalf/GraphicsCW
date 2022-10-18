use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::fmt::Debug;

const DEFAULT_AMBIENT: f32 = 0.2;

#[derive(Debug, Clone)]
pub struct BlinnPhongMaterial {
    ambient: Color,
    diffuse: Color,
    specular: Color,
    power: usize,
}

impl BlinnPhongMaterial {
    pub fn new_from_color(color: Color, shininess: f32) -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient: color * DEFAULT_AMBIENT,
            diffuse: color * (1. - shininess),
            specular: color * shininess,
            power: 20,
        }
    }

    pub fn new_from_colors(
        ambient: Color,
        diffuse: Color,
        specular: Color,
        power: usize,
    ) -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient,
            diffuse,
            specular,
            power,
        }
    }

    pub fn new() -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient: Color::new(0.3, 0.3, 0.3),
            diffuse: Color::new(0.5, 0.5, 0.5),
            specular: Color::new(0.1, 0.1, 0.1),
            power: 20,
        }
    }
}

impl Material for BlinnPhongMaterial {
    fn compute(
        &self,
        view_ray: &Ray,
        hit: &Hit,
        ambient: Color,
        lights: Vec<(Vec3, Color)>,
    ) -> Color {
        lights.iter().fold(
            Color::new(
                self.ambient.red() * ambient.red(),
                self.ambient.green() * ambient.green(),
                self.ambient.blue() * ambient.blue(),
            ),
            |c, (dir, col)| {
                let diffuse = self.diffuse * hit.normal().dot(-*dir).max(0.);
                let reflection_dir: Vec3 = *dir - 2. * (dir.dot(*hit.normal())) * *hit.normal();
                let reflection_dir = reflection_dir.normalize();
                let specular = self.specular
                    * (reflection_dir
                        .dot(-view_ray.direction())
                        .powi(self.power as i32))
                    .max(0.);

                c + col.scale(&(diffuse + specular))
            },
        )
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}
