use crate::color::Color;
use crate::constants::{CAUSTIC_RAD, PHOTON_RAD};
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::photon::{Photon, PhotonType};
use crate::ray::Ray;
use crate::scene::Scene;

// This material adds the photon mapping global illuminations
#[derive(Clone, Debug)]
pub struct AmbientMaterial {}

impl AmbientMaterial {
    pub(crate) fn new() -> AmbientMaterial {
        AmbientMaterial {}
    }
}

impl Material for AmbientMaterial {
    fn compute(&self, _: Ray, hit: &Hit, _: Color, scene: &Scene, _: usize, _: Color) -> Color {
        // Finds all photons within the radius that are indirect photons
        let photons = scene
            .get_photons(*hit.pos(), PHOTON_RAD)
            .into_iter()
            .filter(|p| {
                matches!(p.get_type(), PhotonType::Indirect(_))
                    && p.get_obj() == hit.get_object_index()
            })
            .collect::<Vec<Photon>>();

        // Sums the intensity of all the surrounding photons with a cone shaped
        // average (Jensen photon mapping)
        let photon_map_col =
            photons
                .iter()
                .fold(Color::new_black(), |col, photon| match photon.get_type() {
                    PhotonType::Indirect(c) => {
                        col + c
                            * ((PHOTON_RAD * PHOTON_RAD
                                - photon.get_pos().distance_squared(*hit.pos()))
                                / (PHOTON_RAD * PHOTON_RAD))
                    }
                    _ => panic!("There should not be these types of photon here"),
                });

        // Do the same for caustics, find them in a much smaller radius though
        let caustic_part = scene
            .get_caustics(*hit.pos(), CAUSTIC_RAD)
            .into_iter()
            .fold(Color::new_black(), |c, p| match p.get_type() {
                PhotonType::Caustic(col) => {
                    c + col
                        * ((CAUSTIC_RAD.powi(2) - p.get_pos().distance_squared(*hit.pos()))
                            / (CAUSTIC_RAD.powi(2)))
                        * 0.3
                }
                _ => panic!("Should not be other types of photon in the caustic map"),
            });

        // Divide the photon map light intensity by the square root of the number of
        //  photons, this softens the noise
        photon_map_col * (1. / (photons.len() as f32).sqrt()) + caustic_part
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
