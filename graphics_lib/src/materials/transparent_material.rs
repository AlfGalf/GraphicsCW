use crate::color::Color;
use crate::constants::{
    EPSILON, MAX_PHOTON_RECURSE_DEPTH, MAX_RECURSE_DEPTH, MIN_RECURSE_COEFFICIENT,
};
use crate::hit::Hit;
use crate::materials::material::Material;
use crate::photon::Photon;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Vec3;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct TransparentMaterial {
    refractive_index: f32,
    mat_index: usize,
}

impl<'a> TransparentMaterial {
    pub fn new(refractive_index: f32) -> Self {
        TransparentMaterial {
            refractive_index,
            mat_index: 0,
        }
    }

    fn find_internal_hit(&self, ray: Ray, obj_index: usize, scene: &Scene) -> Option<Hit> {
        let mut intersections = scene
            .intersection(ray)
            .filter(|h| {
                (!h.get_dir()) && h.get_distance() > EPSILON && h.get_object_index() == obj_index
            })
            .collect::<Vec<Hit>>();

        intersections.sort_by(|l, r| l.get_distance().partial_cmp(&r.get_distance()).unwrap());

        intersections.first().cloned()
    }

    fn calc_internal_ray(
        &'a self,
        ray: Ray,
        scene: &Scene,
        recurse_power: Color,
        recurse_depth: usize,
        obj_index: usize,
    ) -> Color {
        if let Some(hit) = self.find_internal_hit(ray, obj_index, scene) {
            let (trans_ray, trans_coeff, refl_ray, refl_coeff) =
                self.find_rays(-*hit.normal(), ray.direction(), *hit.pos(), false);

            let refl_power = recurse_power * refl_coeff;
            let trans_power = recurse_power * trans_coeff;

            let refl_part = if refl_power.max_val() > MIN_RECURSE_COEFFICIENT
                && recurse_depth < MAX_RECURSE_DEPTH
            {
                self.calc_internal_ray(refl_ray, scene, refl_power, recurse_depth + 1, obj_index)
                    * refl_coeff
                // scene.calc_ray(refl_ray, refl_power, recurse_depth + 1).0 * refl_coeff
            } else {
                Color::new_black()
            };

            let trans_part = if trans_power.max_val() > MIN_RECURSE_COEFFICIENT
                && recurse_depth < MAX_RECURSE_DEPTH
            {
                scene
                    .calc_ray(trans_ray.unwrap(), trans_power, recurse_depth + 1)
                    .0
                    * trans_coeff
            } else {
                Color::new_black()
            };

            refl_part + trans_part
        } else {
            scene.calc_ray(ray, recurse_power, recurse_depth + 1).0
        }
    }

    // Finds the ray directions and powers of ray into a transparent material
    fn find_rays(
        &self,
        normal: Vec3,
        incidence: Vec3,
        pos: Vec3,
        going_in: bool,
    ) -> (Option<Ray>, f32, Ray, f32) {
        let refr_index = if going_in {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let incidence = incidence.normalize();
        let normal = normal.normalize();

        let cos_t_i = -(normal).dot(incidence);

        let reflection_dir = incidence + 2. * (cos_t_i) * normal;
        let reflection_dir = reflection_dir.normalize();
        let reflection_ray = Ray::new(pos + reflection_dir * EPSILON, reflection_dir);

        let sin_2_t_i = refr_index.powi(2) * (1. - cos_t_i.powi(2));

        if sin_2_t_i > 1. {
            // Total internal reflection
            return (None, 0., reflection_ray, 1.);
        }

        let sqrt = (1. - sin_2_t_i).sqrt();

        let refracted_dir = refr_index * incidence + (refr_index * cos_t_i - sqrt) * normal;

        let r_floor = ((refr_index * cos_t_i - sqrt) / (refr_index * cos_t_i + sqrt)).powi(2);
        let r_bb = ((cos_t_i - refr_index * sqrt) / (cos_t_i + refr_index * sqrt)).powi(2);

        let r_t_i = ((r_floor + r_bb) / 2.).max(0.).min(1.);
        let t_t_i = 1. - r_t_i;

        let refracted_ray = Ray::new(pos + refracted_dir * EPSILON, refracted_dir);

        (Some(refracted_ray), t_t_i, reflection_ray, r_t_i)
    }

    // Calculates where a photon ends up from refraction
    fn calc_photon_internal(
        &self,
        view_ray: Ray,
        hit: &Hit,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
        light_index: usize,
        inside: bool,
        caustic: bool,
    ) -> Vec<Photon> {
        let (trans_ray, _, refl_ray, refl_coeff) = self.find_rays(
            if inside {
                -*hit.normal()
            } else {
                *hit.normal()
            },
            view_ray.direction(),
            *hit.pos(),
            !inside,
        );

        let mut rng = rand::thread_rng();
        let i: f32 = rng.gen_range((0.)..1.);

        if recurse_depth > MAX_PHOTON_RECURSE_DEPTH {
            // println!("Timeout");
            return vec![];
        }

        // println!("{}", refl_coeff);

        if i < refl_coeff {
            // println!("Refl part");
            // Reflection part
            if inside {
                let Some(new_hit) = self.find_internal_hit(refl_ray, hit.get_object_index(), scene) else {
                    // If refracting internally and doesnt hit an outgoing wall, return nothing
                    // println!("No internal hit");
                    return vec![];
                };

                self.calc_photon_internal(
                    refl_ray,
                    &new_hit,
                    scene,
                    recurse_depth + 1,
                    recurse_power,
                    light_index,
                    true,
                    caustic,
                )
            } else if caustic {
                scene
                    .calculate_caustic(
                        &refl_ray,
                        hit.get_object_index(),
                        light_index,
                        recurse_power,
                        recurse_depth,
                    )
                    .map_or(vec![], |p| vec![p])
            } else {
                scene.calculate_photon_ray(refl_ray, light_index, recurse_depth + 1, recurse_power)
            }
        } else {
            // Transparent part
            if inside {
                // If inside, then the transparent part is outside the object to cast photon into outside world
                if caustic {
                    scene
                        .calculate_caustic(
                            &trans_ray.unwrap(),
                            hit.get_object_index(),
                            light_index,
                            recurse_power,
                            recurse_depth,
                        )
                        .map_or(vec![], |p| vec![p])
                } else {
                    scene.calculate_photon_ray(
                        trans_ray.unwrap(),
                        light_index,
                        recurse_depth + 1,
                        recurse_power,
                    )
                }
            } else {
                // If outside, then cast photon internally
                let Some(new_hit) = self.find_internal_hit(trans_ray.unwrap(), hit.get_object_index(), scene) else {
                    // If doesnt find another material, return nothing
                    return vec![];
                };

                // Find the result of firing another ray internally
                self.calc_photon_internal(
                    trans_ray.unwrap(),
                    &new_hit,
                    scene,
                    recurse_depth + 1,
                    recurse_power,
                    light_index,
                    true,
                    caustic,
                )
            }
        }
    }
}

impl Material for TransparentMaterial {
    fn compute<'a>(
        &'a self,
        view_ray: Ray,
        hit: &'a Hit,
        _: Color,
        scene: &Scene,
        recurse_depth: usize,
        recurse_power: Color,
    ) -> Color {
        let (trans_ray, trans_coeff, refl_ray, refl_coeff) =
            self.find_rays(*hit.normal(), view_ray.direction(), *hit.pos(), true);

        let refl_power = recurse_power * refl_coeff;
        let trans_power = recurse_power * trans_coeff;

        let refl_part = if refl_power.max_val() > MIN_RECURSE_COEFFICIENT
            && recurse_depth < MAX_RECURSE_DEPTH
        {
            scene.calc_ray(refl_ray, refl_power, recurse_depth + 1).0 * refl_coeff
        } else {
            Color::new_black()
        };

        let trans_part = if trans_power.max_val() > MIN_RECURSE_COEFFICIENT
            && recurse_depth < MAX_RECURSE_DEPTH
        {
            self.calc_internal_ray(
                trans_ray.unwrap(),
                scene,
                trans_power,
                recurse_depth,
                hit.get_object_index(),
            ) * trans_coeff
        } else {
            Color::new_black()
        };

        refl_part + trans_part
    }

    fn update_mat_index(&mut self, i: usize) {
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
        self.calc_photon_internal(
            view_ray,
            hit,
            scene,
            recurse_depth,
            recurse_power,
            light_index,
            false,
            false,
        )
    }

    fn needs_caustic(&self) -> bool {
        true
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
        // Return the first photon from sending in a caustic ray
        self.calc_photon_internal(
            view_ray,
            hit,
            scene,
            recurse_depth,
            color,
            light_index,
            false,
            true,
        )
        .into_iter()
        .next()
    }
}
