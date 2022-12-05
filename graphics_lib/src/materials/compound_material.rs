use crate::color::Color;
use crate::constants::DEFAULT_AMBIENT;
use crate::hit::Hit;
use crate::materials::ambient_material::AmbientMaterial;
use crate::materials::diffuse_material::DiffuseMaterial;
use crate::materials::material::Material;
use crate::materials::reflective_material::ReflectiveMaterial;
use crate::materials::specular_material::SpecularMaterial;
use crate::materials::transparent_material::TransparentMaterial;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;
use rand::Rng;
use std::fmt::Debug;

// This material represents several other materials, each with their own
//      weighting that are applied and summed together
#[derive(Debug)]
pub struct CompoundMaterial {
    color: Color,
    materials: Vec<(Box<dyn Material + Sync + Send>, f32)>,
    mat_index: usize,
}

impl CompoundMaterial {
    pub fn new(materials: Vec<(Box<dyn Material + Sync + Send>, f32)>, color: Color) -> Self {
        let scale = 1. / materials.iter().fold(0., |ct, (_, weight)| ct + weight);

        CompoundMaterial {
            color,
            materials: materials
                .into_iter()
                .map(|(m, w)| (m, (w * scale)))
                .collect(),
            mat_index: 0,
        }
    }

    pub fn new_matte_material(col: Color, specular: f32) -> CompoundMaterial {
        assert!(1. >= specular);
        assert!(0. <= specular);
        CompoundMaterial::new(
            vec![
                (Box::new(AmbientMaterial::new()), DEFAULT_AMBIENT),
                (
                    Box::new(DiffuseMaterial::new()),
                    (1. - DEFAULT_AMBIENT) * (1. - specular),
                ),
                (
                    Box::new(SpecularMaterial::new(10)),
                    (1. - DEFAULT_AMBIENT) * (specular),
                ),
            ],
            col,
        )
    }

    pub fn new_reflective_material(col: Color, reflectivity: f32) -> CompoundMaterial {
        assert!(1. >= reflectivity);
        assert!(0. <= reflectivity);
        CompoundMaterial::new(
            vec![
                (
                    Box::new(CompoundMaterial::new_matte_material(
                        Color::new_grey(1.),
                        0.2,
                    )),
                    (1. - reflectivity),
                ),
                (Box::new(ReflectiveMaterial::new()), (reflectivity)),
            ],
            col,
        )
    }

    pub fn new_transparent_material(refractive_index: f32) -> CompoundMaterial {
        CompoundMaterial::new(
            vec![
                (
                    Box::new(CompoundMaterial::new_matte_material(
                        Color::new_grey(1.),
                        0.1,
                    )),
                    0.1,
                ),
                (
                    Box::new(TransparentMaterial::new(refractive_index)),
                    1. - 0.1,
                ),
            ],
            Color::new_grey(1.),
        )
    }

    pub fn new_transparent_material_opacity(
        refractive_index: f32,
        color: Color,
        opacity: f32,
    ) -> CompoundMaterial {
        CompoundMaterial::new(
            vec![
                (
                    Box::new(CompoundMaterial::new_matte_material(
                        Color::new_grey(1.),
                        0.1,
                    )),
                    0.1,
                ),
                (
                    Box::new(TransparentMaterial::new(refractive_index)),
                    1. - opacity,
                ),
                (Box::new(DiffuseMaterial::new()), opacity),
            ],
            color,
        )
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
            .fold(Color::new_black(), |tc, (m, w)| {
                tc + m
                    .compute(
                        view_ray,
                        hit,
                        ambient,
                        scene,
                        recurse_depth,
                        recurse_power.scale(&self.color) * *w,
                    )
                    .scale(&self.color)
                    * *w
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

    fn compute_photon(
        &self,
        view_ray: Ray,
        hit: &Hit,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
        light_index: usize,
    ) -> Vec<Photon> {
        let mut rng = rand::thread_rng();
        let mut i: f32 = rng.gen_range((0.)..1.);

        let mat = {
            let mut res = self.materials.first().unwrap();
            for mat in self.materials.iter() {
                i -= mat.1;
                if i <= 0. {
                    res = mat;
                    break;
                }
            }
            res
        };

        let mut res = mat.0.compute_photon(
            view_ray,
            hit,
            scene,
            recurse_depth,
            recurse_power.scale_const_mag(&self.color),
            light_index,
        );
        res.push(Photon::new_indirect(
            *hit.pos(),
            light_index,
            recurse_power,
            hit.get_object_index(),
        ));
        res
    }

    fn needs_caustic(&self) -> bool {
        self.materials.iter().any(|(m, _)| m.needs_caustic())
    }

    fn compute_caustic_ray(
        &self,
        view_ray: Ray,
        hit: &Hit,
        scene: &Scene,
        recurse_depth: usize,
        light_index: usize,
        color: Color,
    ) -> Option<Photon> {
        // If any sub material retransmits caustics, use that
        self.materials
            .iter()
            .filter_map(|(m, _)| {
                m.compute_caustic_ray(
                    view_ray,
                    hit,
                    scene,
                    recurse_depth,
                    light_index,
                    color.scale(&self.color),
                )
            })
            .next()
    }
}
