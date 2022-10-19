extern crate graphics_lib;

use graphics_lib::frame_buffer::FrameBuffer;

use graphics_lib::line_drawer::draw_line;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let mut fb = FrameBuffer::new(512, 256);

    draw_line(&mut fb, 10, 0, 10, 200);
    draw_line(&mut fb, 400, 0, 400, 200);

    draw_line(&mut fb, 40, 30, 20, 20);
    draw_line(&mut fb, 40, 20, 20, 30);

    File::write_all(&mut file, &(fb.to_rgb_file())).unwrap();
}
