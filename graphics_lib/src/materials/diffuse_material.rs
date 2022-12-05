use crate::color::Color;
use crate::fibonacci_spiral::hemisphere_random;
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;

// This material represents the diffuse lighting of a material
//      Uses the Phong lighting model
#[derive(Debug, Clone)]
pub struct DiffuseMaterial {
    mat_index: usize,
}

impl DiffuseMaterial {
    pub fn new() -> DiffuseMaterial {
        DiffuseMaterial { mat_index: 0 }
    }
}

impl Material for DiffuseMaterial {
    fn compute<'a>(&self, _: Ray, hit: &Hit, _: Color, scene: &Scene, _: usize, _: Color) -> Color {
        scene
            .get_lights()
            .iter()
            .enumerate()
            .fold(Color::new_black(), |c, (i, light)| {
                let intensity = light.get_intensity(*hit.pos(), scene, i);

                let dir = light.get_direction(*hit.pos());

                let diffuse = hit.normal().dot(-dir).max(0.);

                c + intensity * diffuse
            })
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
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
        light_index: usize,
    ) -> Vec<Photon> {
        // New photon in random direction (acute angle to normal)
        let dir = hemisphere_random(*hit.normal());
        scene.calculate_photon_ray(
            Ray::new(*hit.pos(), dir),
            light_index,
            recurse_depth,
            recurse_power,
        )
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
        _: Color,
    ) -> Option<Photon> {
        // Does not retransmit caustics
        None
    }
}
