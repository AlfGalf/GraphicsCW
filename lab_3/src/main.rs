extern crate graphics_lib;

use glam::{Affine3A, Vec3};
use graphics_lib::camera::Camera;
use graphics_lib::color::Color;
use graphics_lib::poly_mesh::PolyMesh;
use graphics_lib::scene::{Object, ObjectEnum, Scene};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let scene = Scene {
        objects: vec![
            Object::new_poly(
                PolyMesh::from_file(
                    BufReader::new(File::open("teapot_smaller.ply").unwrap()),
                    false,
                )
                .unwrap()
                .apply_transform(Affine3A::from_cols_array(&[
                    1.0, 0.0, 0.0, //1
                    0.0, 0.0, 1.0, //2
                    0.0, 1.0, 0.0, // 3
                    0.0, -2.0, 0.0, //
                ])),
                Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
            ),
            Object::new_sphere(
                Vec3::new(1., 1., -0.5),
                1.,
                Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
            ),
        ],
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
