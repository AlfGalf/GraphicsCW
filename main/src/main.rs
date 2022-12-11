extern crate graphics_lib;

use glam::Vec3;
use graphics_lib::cameras::normal_camera::NormalCamera;
use graphics_lib::color::Color;
use graphics_lib::lights::point_light::PointLight;
use graphics_lib::materials::compound_material::CompoundMaterial;
use graphics_lib::materials::material::Material;
use graphics_lib::objects::plane::Plane;
use graphics_lib::objects::sphere::Sphere;
use graphics_lib::scene::Scene;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let materials: Vec<Box<dyn Material + Sync + Send>> = vec![
        Box::new(CompoundMaterial::new_reflective_material(
            // 0 -> reflective slight green
            Color::new(1.0, 1.0, 1.0),
            0.9,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 1 -> blue matte
            Color::new(0.3, 0.3, 0.8),
            0.4,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 2 -> blue slightly reflective
            Color::new(0.8, 0.3, 0.3),
            0.3,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 3 -> grey matte
            Color::new(0.5, 0.5, 0.5),
            0.4,
        )),
    ];

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let plane_bottom = Plane::new(Vec3::new(0., -2., 0.), Vec3::new(0., 1., 0.), 3);

    let plane_back = Plane::new(Vec3::new(0., 0., 4.), Vec3::new(0., 0., -1.), 3);

    let plane_left = Plane::new(Vec3::new(-4., 0., 0.), Vec3::new(1., 0., 0.), 1);

    let plane_right = Plane::new(Vec3::new(4., 0., 0.), Vec3::new(-1., 0., 0.), 2);

    let plane_top = Plane::new(Vec3::new(0., 4., 0.), Vec3::new(0., -1., 0.), 3);

    let plane_front = Plane::new(Vec3::new(0., 0., -25.), Vec3::new(0., 0., 1.), 3);

    let main_light = PointLight::new(Vec3::new(1.0, 3.5, -5.0), Color::new(0.9, 0.9, 0.9));

    let sphere = Sphere::new(Vec3::new(0., 1., 0.), 1.2, 0);

    let scene = Scene::new(
        vec![
            Box::new(plane_back),
            Box::new(plane_right),
            Box::new(plane_left),
            Box::new(plane_top),
            Box::new(plane_bottom),
            Box::new(plane_front),
            Box::new(sphere),
        ],
        vec![Box::new(main_light)],
        materials,
        Box::new(NormalCamera::new(
            Vec3::new(1., 1., -20.),
            Vec3::new(-0.05, 0., 1.0),
            Vec3::new(0., 1., 0.),
            1.7,
        )),
        // Box::new(DoFCamera::new(
        //     Vec3::new(3., 0., -20.),
        //     Vec3::new(-0.10, 0., 1.0),
        //     Vec3::new(0., 1., 0.),
        //     1.7,
        //     10,
        //     20.,
        //     0.5,
        // )),
    );

    let fb = scene.render(400, 400);

    File::write_all(&mut file, &(fb.to_rgb_file(0.5))).unwrap();
}
