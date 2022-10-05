extern crate graphics_lib;

use graphics_lib::frame_buffer::FrameBuffer;

use graphics_lib::camera::Camera;
use graphics_lib::color::Color;
use graphics_lib::line_drawer::draw_line;
use graphics_lib::poly_mesh::PolyMesh;
use graphics_lib::scene::{Object, Scene};
use graphics_lib::transform::Transform;
use graphics_lib::vector::Vector;
use graphics_lib::vertex::Vertex;
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
        objects: vec![Object::Poly {
            poly: PolyMesh::from_file(
                BufReader::new(File::open("teapot_smaller.ply").unwrap()),
                false,
            )
            .unwrap()
            .apply_transform(&Transform::new(
                1.0, 0.0, 0.0, 0.0, // x
                0.0, 0.0, 1.0, -2.0, // y
                0.0, 1.0, 0.0, 0.0, // z
                0.0, 0.0, 0.0, 1.0,
            )),
            color: Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            },
        }],
        lights: vec![],
        camera: Camera {
            position: Vertex(0., 0., -20.),
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            focal_length: 2.0,
        },
    };

    let fb = scene.render(960, 540);

    File::write_all(&mut file, &(fb.to_rgb_file())).unwrap();
}
