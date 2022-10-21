use crate::color::Color;
use crate::constants::DEFAULT_AMBIENT;
use crate::hit::Hit;
use crate::materials::ambient_material::AmbientMaterial;
use crate::materials::diffuse_material::DiffuseMaterial;
use crate::materials::material::Material;
use crate::materials::reflective_material::ReflectiveMaterial;
use crate::materials::specular_material::SpecularMaterial;
use crate::ray::Ray;
use crate::scene::Scene;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct CompoundMaterial {
    materials: Vec<(Box<dyn Material + Sync>, Color)>,
}

impl CompoundMaterial {
    pub fn new(materials: Vec<(Box<dyn Material + Sync>, Color)>) -> Self {
        let scale = 1.
            / materials
                .iter()
                .fold(Color::new_black(), |ct, (_, col)| ct + *col)
                .max_val();
        CompoundMaterial {
            materials: materials
                .into_iter()
                .map(|(m, c)| (m, (c * scale)))
                .collect(),
        }
    }

    pub fn new_matte_material(col: Color, specular: f32) -> CompoundMaterial {
        assert!(1. >= specular);
        assert!(0. <= specular);
        CompoundMaterial::new(vec![
            (Box::new(AmbientMaterial::new()), col * DEFAULT_AMBIENT),
            (
                Box::new(DiffuseMaterial::new()),
                col * (1. - DEFAULT_AMBIENT) * (1. - specular),
            ),
            (
                Box::new(SpecularMaterial::new(10)),
                col * (1. - DEFAULT_AMBIENT) * (specular),
            ),
        ])
    }

    pub fn new_reflective_material(col: Color, reflectivity: f32) -> CompoundMaterial {
        assert!(1. >= reflectivity);
        assert!(0. <= reflectivity);
        CompoundMaterial::new(vec![
            (Box::new(AmbientMaterial::new()), col * DEFAULT_AMBIENT),
            (
                Box::new(DiffuseMaterial::new()),
                col * (1. - DEFAULT_AMBIENT) * (1. - reflectivity),
            ),
            (
                Box::new(ReflectiveMaterial::new()),
                col * (1. - DEFAULT_AMBIENT) * (reflectivity),
            ),
        ])
    }
}

impl Material for CompoundMaterial {
    fn compute(
        &self,
        view_ray: &Ray,
        hit: &Hit,
        ambient: Color,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Color {
        self.materials
            .iter()
            .fold(Color::new_black(), |tc, (m, c)| {
                tc + m
                    .compute(
                        view_ray,
                        hit,
                        ambient,
                        scene,
                        recurse_depth,
                        recurse_power.scale(c),
                    )
                    .scale(c)
            })
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}
