extern crate graphics_lib;

use graphics_lib::frame_buffer::FrameBuffer;

use glam::Affine3A;
use graphics_lib::line_drawer::draw_line;
use graphics_lib::poly_mesh::PolyMesh;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let mut fm = FrameBuffer::new(1024, 1024);

    match PolyMesh::from_file(
        BufReader::new(File::open("../../lab_3/teapot_smaller.ply").unwrap()),
        false,
    )
    .map(|mut p| {
        p.apply_transform(Affine3A::from_cols_array(&[
            2.0, 0.0, 0.0, //0.0, //1
            0.0, 0.0, 2.0, //-8.0, //2
            0.0, 2.0, 0.0, //18.0, //3
            0.0, -8.0, 18.0, //0.0,
        ]));
        p
    }) {
        Err(e) => println!("Error: {}", e),

        Ok(p) => {
            for t in p.triangles {
                let p1 = p.vertices.get(t.0).unwrap();
                let p2 = p.vertices.get(t.1).unwrap();
                let p3 = p.vertices.get(t.2).unwrap();

                let x0 = (p1.x / p1.z) * 700. + 512.;
                let y0 = (p1.y / p1.z) * -700. + 256.;
                let x1 = (p2.x / p2.z) * 700. + 512.;
                let y1 = (p2.y / p2.z) * -700. + 256.;
                let x2 = (p3.x / p3.z) * 700. + 512.;
                let y2 = (p3.y / p3.z) * -700. + 256.;

                draw_line(&mut fm, x0 as usize, y0 as usize, x1 as usize, y1 as usize);
                draw_line(&mut fm, x1 as usize, y1 as usize, x2 as usize, y2 as usize);
                draw_line(&mut fm, x2 as usize, y2 as usize, x0 as usize, y0 as usize);
            }
        }
    }
    File::write_all(&mut file, &(fm.to_rgb_file())).unwrap();
}
