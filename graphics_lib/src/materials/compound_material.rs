use crate::color::Color;
use crate::constants::DEFAULT_AMBIENT;
use crate::hit::Hit;
use crate::materials::ambient_material::AmbientMaterial;
use crate::materials::diffuse_material::DiffuseMaterial;
use crate::materials::material::Material;
use crate::materials::reflective_material::ReflectiveMaterial;
use crate::materials::specular_material::SpecularMaterial;
use crate::materials::trasparent_material::TransparentMaterial;
use crate::ray::Ray;
use crate::scene::Scene;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CompoundMaterial {
    materials: Vec<(Box<dyn Material + Sync + Send>, Color)>,
    mat_index: usize,
}

impl CompoundMaterial {
    pub fn new(materials: Vec<(Box<dyn Material + Sync + Send>, Color)>) -> Self {
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
            mat_index: 0,
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

    pub fn new_transparent_material(refractive_index: f32) -> CompoundMaterial {
        CompoundMaterial::new(vec![
            (
                Box::new(AmbientMaterial::new()),
                Color::new_grey(DEFAULT_AMBIENT),
            ),
            (
                Box::new(TransparentMaterial::new(refractive_index)),
                Color::new_grey(1. - DEFAULT_AMBIENT),
            ),
        ])
    }

    pub fn new_transparent_material_opacity(
        refractive_index: f32,
        color: Color,
        opacity: f32,
    ) -> CompoundMaterial {
        CompoundMaterial::new(vec![
            (
                Box::new(AmbientMaterial::new()),
                Color::new_grey(DEFAULT_AMBIENT),
            ),
            (
                Box::new(TransparentMaterial::new(refractive_index)),
                color * ((1. - DEFAULT_AMBIENT) * (1. - opacity)),
            ),
            (
                Box::new(DiffuseMaterial::new()),
                color * ((1. - DEFAULT_AMBIENT) * (opacity)),
            ),
        ])
    }
}

impl Material for CompoundMaterial {
    fn compute<'a>(
        &'a self,
        view_ray: Ray,
        hit: &'a Hit,
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

    fn update_mat_index(&mut self, i: usize) {
        self.materials
            .iter_mut()
            .for_each(|m| m.0.update_mat_index(i));
        self.mat_index = i
    }

    fn get_mat_index(&self) -> usize {
        self.mat_index
    }
}
