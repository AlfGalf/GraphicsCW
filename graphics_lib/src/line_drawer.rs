use crate::frame_buffer::FrameBuffer;

fn draw_x_line(fb: &mut FrameBuffer, sx: isize, sy: isize, ex: isize, ey: isize) {
    let dir: isize = if sx > ex { -1 } else { 1 };
    let change: isize = if sy > ey { -1 } else { 1 };

    let mut wy = sy;
    let mut x = sx;

    let dx = if ex > sx { ex - sx } else { sx - ex };
    let dy = if ey > sy { ey - sy } else { sy - ey };

    let mut fy = dy / 2;

    while (dir == 1 && x <= ex) || (dir == -1 && x >= ex) {
        fb.plot_pixel(x as usize, wy as usize, 1.0, 1.0, 1.0);
        x += dir;

        fy += dy;

        if fy > dx {
            wy += change;
            fy -= dx;
        }
    }
}

fn draw_y_line(fb: &mut FrameBuffer, sx: isize, sy: isize, ex: isize, ey: isize) {
    let dir: isize = if sy > ey { -1 } else { 1 };
    let change: isize = if sx > ex { -1 } else { 1 };

    let mut wx = sx;
    let mut y = sy;

    let dx = if ex > sx { ex - sx } else { sx - ex };
    let dy = if ey > sy { ey - sy } else { sy - ey };

    let mut fx = dx / 2;

    while (dir == 1 && y <= ey) || (dir == -1 && y >= ey) {
        fb.plot_pixel(wx as usize, y as usize, 1.0, 1.0, 1.0);
        y += dir;

        fx += dx;

        if fx > dy {
            wx += change;
            fx -= dy;
        }
    }
}

pub fn draw_line(fb: &mut FrameBuffer, sx: usize, sy: usize, ex: usize, ey: usize) {
    // assert!(0 <= sx);
    // assert!(0 <= ex);
    // assert!(0 <= sy);
    // assert!(0 <= ey);
    // assert!(sx < fb.width);
    // assert!(ex < fb.width);
    // assert!(sy < fb.height);
    // assert!(ey < fb.height);
    // let sx = sx.max(0).min(fb.width - 1);
    // let ex = ex.max(0).min(fb.width - 1);
    // let sy = sy.max(0).min(fb.height - 1);
    // let ey = ey.max(0).min(fb.height - 1);

    let dx = if ex == sx {
        0
    } else if ex > sx {
        ex - sx - 1
    } else {
        sx - ex - 1
    };

    let dy = if ey == sy {
        0
    } else if ey > sy {
        ey - sy - 1
    } else {
        sy - ey - 1
    };

    if (dx == 0) && (dy == 0) {
        fb.plot_pixel(sx, sy, 1.0, 1.0, 1.0);
    } else if dx >= dy {
        draw_x_line(fb, sx as isize, sy as isize, ex as isize, ey as isize);
    } else {
        draw_y_line(fb, sx as isize, sy as isize, ex as isize, ey as isize);
    }
}
