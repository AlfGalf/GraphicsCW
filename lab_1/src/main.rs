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

    let mut fm = FrameBuffer::new(512, 256);

    draw_line(&mut fm, 0, 0, 10, 20);
    draw_line(&mut fm, 0, 20, 10, 0);

    draw_line(&mut fm, 40, 30, 20, 20);
    draw_line(&mut fm, 40, 20, 20, 30);

    File::write_all(&mut file, &(fm.to_rgb_file())).unwrap();
}
