use crate::color::Color;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::photon::PhotonType::Caustic;
use crate::photon::{Photon, PhotonType};
use crate::ray::Ray;
use crate::scene::Scene;

const PHOTON_RAD: f32 = 0.8;
const CAUSTIC_RAD: f32 = 0.05;

// This material adds the photon mapping global illuminations
#[derive(Clone, Debug)]
pub struct AmbientMaterial {
    mat_index: usize,
}

impl AmbientMaterial {
    pub(crate) fn new() -> AmbientMaterial {
        AmbientMaterial { mat_index: 0 }
    }
}

impl Material for AmbientMaterial {
    fn compute(&self, _: Ray, hit: &Hit, _: Color, scene: &Scene, _: usize, _: Color) -> Color {
        let mut photons = scene
            .get_photons(*hit.pos(), PHOTON_RAD)
            .into_iter()
            .filter(|p| {
                !matches!(p.get_type(), PhotonType::Shadow) && p.get_obj() == hit.get_object_index()
            })
            .collect::<Vec<Photon>>();

        photons.sort_by(|p1, p2| {
            hit.pos()
                .distance(p1.get_pos())
                .total_cmp(&hit.pos().distance(p2.get_pos()))
        });

        let col = photons
            .iter()
            .fold(Color::new_black(), |col, photon| match photon.get_type() {
                PhotonType::Direct(c) => {
                    col + c
                        * ((PHOTON_RAD * PHOTON_RAD
                            - photon.get_pos().distance_squared(*hit.pos()))
                            / (PHOTON_RAD * PHOTON_RAD))
                }
                PhotonType::Indirect(c) => {
                    col + c
                        * ((PHOTON_RAD * PHOTON_RAD
                            - photon.get_pos().distance_squared(*hit.pos()))
                            / (PHOTON_RAD * PHOTON_RAD))
                }
                PhotonType::Shadow => col,
                PhotonType::Caustic(_) => panic!("There should not be caustics here"),
            });

        let caustic_part = scene
            .get_caustics(*hit.pos(), CAUSTIC_RAD)
            .into_iter()
            .fold(Color::new_black(), |c, p| match p.get_type() {
                Caustic(col) => {
                    c + col
                        * ((CAUSTIC_RAD.powi(2) - p.get_pos().distance_squared(*hit.pos()))
                            / (CAUSTIC_RAD.powi(2)))
                        * 0.3
                }
                _ => panic!("Should not be other types of photon in the caustic map"),
            });

        col * (1. / (photons.len() as f32).sqrt()) + caustic_part
    }

    fn update_mat_index(&mut self, i: usize) {
        self.mat_index = i
    }

    fn get_mat_index(&self) -> usize {
        self.mat_index
    }

    fn compute_photon(
        &self,
        _: Ray,
        hit: &Hit,
        _: &Scene,
        _: usize,
        recurse_power: Color,
        light_index: usize,
    ) -> Vec<Photon> {
        // Absorb photon
        // println!("test");
        vec![Photon::new_indirect(
            *hit.pos(),
            light_index,
            recurse_power,
            hit.get_object_index(),
        )]
    }

    fn needs_caustic(&self) -> bool {
        false
    }

    fn compute_caustic_ray(
        &self,
        _view_ray: Ray,
        _hit: &Hit,
        _scene: &Scene,
        _recurse_depth: usize,
        _light_index: usize,
        _color: Color,
    ) -> Option<Photon> {
        // Does not retransmit caustics
        None
    }
}
