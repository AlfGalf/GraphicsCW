use crate::color::Color;
use crate::hit::Hit;
use crate::lights::light::Light;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;
use std::fmt::Debug;

const DEFAULT_AMBIENT: f32 = 0.2;
const MAX_REFLECTIONS: usize = 3;
const MIN_REFLECTION_COEFF: f32 = 1.0E-2;
const EPSILON: f32 = 1.0E-4;

#[derive(Debug, Clone)]
pub struct BlinnPhongMaterial {
    ambient: Color,
    diffuse: Color,
    specular: Color,
    reflection: Color,
    power: usize,
}

impl BlinnPhongMaterial {
    pub fn new_from_color(color: Color, shininess: f32, reflection: f32) -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient: color * DEFAULT_AMBIENT * (1. - reflection),
            diffuse: color * (1. - shininess) * (1. - reflection),
            specular: color * shininess * (1. - reflection),
            reflection: Color::new(reflection, reflection, reflection),
            power: 20,
        }
    }

    pub fn new_from_colors(
        ambient: Color,
        diffuse: Color,
        specular: Color,
        reflection: Color,
        power: usize,
    ) -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient,
            diffuse,
            specular,
            power,
            reflection,
        }
    }

    pub fn new() -> BlinnPhongMaterial {
        BlinnPhongMaterial {
            ambient: Color::new(0.3, 0.3, 0.3),
            diffuse: Color::new(0.5, 0.5, 0.5),
            specular: Color::new(0.1, 0.1, 0.1),
            reflection: Color::new(0.1, 0.1, 0.1),
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
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Color {
        let reflection_dir: Vec3 =
            view_ray.direction() - 2. * (view_ray.direction().dot(*hit.normal())) * *hit.normal();
        let reflection_dir = reflection_dir.normalize();
        let s_d_a_comp = scene.lights.iter().fold(
            Color::new(
                self.ambient.red() * ambient.red(),
                self.ambient.green() * ambient.green(),
                self.ambient.blue() * ambient.blue(),
            ),
            |c, light| {
                let intensity = light.get_intensity(hit.pos(), scene);
                let dir = light.get_direction(hit.pos());

                let diffuse = self.diffuse * hit.normal().dot(-dir).max(0.);
                let specular =
                    self.specular * (reflection_dir.dot(-dir).powi(self.power as i32)).max(0.);

                c + intensity.scale(&(diffuse + specular))
            },
        );
        let new_recurse_power = recurse_power.scale(&self.reflection);
        if recurse_depth < MAX_REFLECTIONS && new_recurse_power.min_val() > MIN_REFLECTION_COEFF {
            let reflection_ray = Ray::new(*hit.pos() + reflection_dir * EPSILON, reflection_dir);
            let res = scene.calc_ray(&reflection_ray, new_recurse_power, recurse_depth + 1);

            s_d_a_comp + res.0.scale(&new_recurse_power)
        } else {
            s_d_a_comp
        }
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}
