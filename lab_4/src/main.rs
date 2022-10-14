extern crate graphics_lib;

use glam::{Affine3A, Vec3};
use graphics_lib::camera::Camera;
use graphics_lib::color::Color;
use graphics_lib::lights::directional_light::DirectionalLight;
use graphics_lib::materials::blinn_phong_material::BlinnPhongMaterial;
use graphics_lib::objects::object::Object;
use graphics_lib::objects::plane::Plane;
use graphics_lib::objects::poly_mesh::PolyMesh;
use graphics_lib::objects::sphere::Sphere;
use graphics_lib::scene::Scene;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let mut teapot = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.ply").unwrap()),
        BlinnPhongMaterial::new_from_color(Color::new(0., 0.5, 1.0), 0.6),
        // FalseColorMaterial::new(),
        false,
    )
    .unwrap();

    teapot.apply_transform(&Affine3A::from_cols_array(&[
        1.0, 0.0, 0.0, //1
        0.0, 0.0, 1.0, //2
        0.0, 1.0, 0.0, // 3
        0.0, -2.0, 0.0, //
    ]));

    let sphere = Sphere::new(
        Vec3::new(1., 1., -0.5),
        1.,
        BlinnPhongMaterial::new_from_color(Color::new(1., 0.5, 0.5) * 0.5, 0.6),
        // FalseColorMaterial::new(),
    );

    let plane = Plane::new(
        Vec3::new(0., -2., 0.),
        Vec3::new(0., 1., 0.),
        BlinnPhongMaterial::new_from_color(Color::new(0.2, 0.8, 0.2), 0.5),
        // FalseColorMaterial::new(),
    );

    let plane_2 = Plane::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., -1.),
        // FalseColorMaterial::new(),
        BlinnPhongMaterial::new_from_color(Color::new(0.5, 0.5, 0.5), 0.4),
    );

    let light = DirectionalLight::new(Vec3::new(6.0, -10.0, 6.0), Color::new(0.9, 0.3, 0.3));
    let light_2 = DirectionalLight::new(Vec3::new(-6.0, -10.0, 6.0), Color::new(0.3, 0.3, 0.9));

    let scene = Scene {
        objects: vec![
            Box::new(teapot),
            Box::new(sphere),
            Box::new(plane),
            Box::new(plane_2),
        ],
        lights: vec![Box::new(light), Box::new(light_2)],
        camera: Camera {
            position: Vec3::new(0., 0., -20.),
            direction: Vec3::new(0.05, 0.0, 1.0),
            up: Vec3::new(0., 1., 0.),
            focal_length: 2.0,
        },
    };

    let fb = scene.render(960, 540);

    File::write_all(&mut file, &(fb.to_rgb_file())).unwrap();
}
