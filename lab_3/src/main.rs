extern crate graphics_lib;

use glam::{Affine3A, Vec3};
use graphics_lib::camera::Camera;
use graphics_lib::color::Color;
use graphics_lib::false_color_material::FalseColorMaterial;
use graphics_lib::object::Object;
use graphics_lib::poly_mesh::PolyMesh;
use graphics_lib::scene::Scene;
use graphics_lib::simple_color_material::SimpleColorMaterial;
use graphics_lib::sphere::Sphere;
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
        BufReader::new(File::open("teapot_smaller.ply").unwrap()),
        FalseColorMaterial {},
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
        SimpleColorMaterial::new(Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        }),
    );

    let scene = Scene {
        objects: vec![Box::new(teapot), Box::new(sphere)],
        lights: vec![],
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
