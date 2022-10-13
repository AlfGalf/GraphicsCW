use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Pixel {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub depth: f32,
}

impl Pixel {
    pub fn new() -> Self {
        Pixel {
            red: 0.,
            green: 0.,
            blue: 0.,
            depth: 0.,
        }
    }

    pub fn from_colors(r: f32, g: f32, b: f32, d: f32) -> Self {
        Pixel {
            red: r,
            green: g,
            blue: b,
            depth: d,
        }
    }

    pub fn from_color(color: Color, d: f32) -> Self {
        Pixel {
            red: color.red,
            green: color.green,
            blue: color.blue,
            depth: d,
        }
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    frame_buffer: Vec<Pixel>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            width: width,
            height: height,
            frame_buffer: vec![Pixel::new(); width * height],
        }
    }

    pub fn plot_pixel(&mut self, x: usize, y: usize, red: f32, green: f32, blue: f32) {
        let x = x.max(0).min(self.width - 1);
        let y = y.max(0).min(self.height - 1);
        self.frame_buffer[y * self.width + x].red = red;
        self.frame_buffer[y * self.width + x].green = green;
        self.frame_buffer[y * self.width + x].blue = blue;
    }

    pub fn plot_depth(&mut self, x: usize, y: usize, depth: f32) {
        let x = x.max(0).min(self.width - 1);
        let y = y.max(0).min(self.height - 1);
        self.frame_buffer[y * self.width + x].depth = depth;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Pixel {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.frame_buffer[y * self.width + x]
    }

    pub fn to_rgb_file(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        let max_val = self.frame_buffer.iter().fold(0., |prev: f32, pixel| {
            prev.max(pixel.red).max(pixel.green).max(pixel.blue)
        });
        let min_val = self.frame_buffer.iter().fold(0., |prev: f32, pixel| {
            prev.min(pixel.red).min(pixel.green).min(pixel.blue)
        });

        let diff = if max_val - min_val == 0. {
            1.
        } else {
            max_val - min_val
        };

        output.append(&mut "P6\n".as_bytes().to_vec());
        output.append(
            &mut format!("{} {}\n255\n", self.width, self.height)
                .as_bytes()
                .to_vec(),
        );

        self.frame_buffer.iter().for_each(|pixel| {
            output.push(u8::try_from((((pixel.red - min_val) / diff) * 255.) as usize).unwrap());
            output.push(u8::try_from(((pixel.green * 255.) / diff) as usize).unwrap());
            output.push(u8::try_from(((pixel.blue * 255.) / diff) as usize).unwrap());
        });

        output
    }

    pub fn to_depth_file(&self) -> String {
        todo!()
    }
}
