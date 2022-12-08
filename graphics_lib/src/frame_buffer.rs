use crate::color::Color;

#[derive(Clone, Debug)]
pub(crate) struct Pixel {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub depth: f32,
}

// Represents a pixel in a scene
impl Pixel {
    pub(crate) fn new() -> Self {
        Pixel {
            red: 0.,
            green: 0.,
            blue: 0.,
            depth: 0.,
        }
    }

    pub(crate) fn from_color(color: Color, d: f32) -> Self {
        Pixel {
            red: color.red(),
            green: color.green(),
            blue: color.blue(),
            depth: d,
        }
    }
}

// Frame buffer to store image before outputting
#[derive(Debug)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    frame_buffer: Vec<Pixel>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            width,
            height,
            frame_buffer: vec![Pixel::new(); width * height],
        }
    }

    // Adds color for a pixel
    pub fn plot_pixel(&mut self, x: usize, y: usize, red: f32, green: f32, blue: f32) {
        let x = x.max(0).min(self.width - 1);
        let y = y.max(0).min(self.height - 1);
        self.frame_buffer[y * self.width + x].red = red;
        self.frame_buffer[y * self.width + x].green = green;
        self.frame_buffer[y * self.width + x].blue = blue;
    }

    // Adds depth imformation for a pixel
    pub fn plot_depth(&mut self, x: usize, y: usize, depth: f32) {
        let x = x.max(0).min(self.width - 1);
        let y = y.max(0).min(self.height - 1);
        self.frame_buffer[y * self.width + x].depth = depth;
    }

    // Outputs a a stream of bytes that makeup a PPM file for image
    pub fn to_rgb_file(&self, cap: f32) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        // Square roots every pixel in the image and caps the brightness
        let out_pixels: Vec<Pixel> = self
            .frame_buffer
            .iter()
            .map(|p| {
                Pixel::from_color(
                    Color::new(
                        p.red.min(cap).sqrt(),
                        p.green.min(cap).sqrt(),
                        p.blue.min(cap).sqrt(),
                    ),
                    p.depth,
                )
            })
            .collect();

        // Finds maximum and minimum values to scale the image by
        let max_val = out_pixels.iter().fold(f32::MIN, |prev: f32, pixel| {
            prev.max(pixel.red).max(pixel.green).max(pixel.blue)
        });
        let min_val = out_pixels.iter().fold(f32::MAX, |prev: f32, pixel| {
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

        out_pixels.iter().for_each(|pixel| {
            output.push(u8::try_from((((pixel.red - min_val) / diff) * 255.) as usize).unwrap());
            output.push(u8::try_from((((pixel.green - min_val) / diff) * 255.) as usize).unwrap());
            output.push(u8::try_from((((pixel.blue - min_val) / diff) * 255.) as usize).unwrap());
        });

        output
    }

    // Outputs a byte stream for a PPM file of the depth of the image
    pub fn to_depth_file(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        let max_val = self
            .frame_buffer
            .iter()
            .fold(f32::MIN, |prev: f32, pixel| prev.max(pixel.depth));
        let min_val = self
            .frame_buffer
            .iter()
            .fold(f32::MAX, |prev: f32, pixel| prev.min(pixel.depth));

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
            output.push(u8::try_from((((max_val - pixel.depth) / diff) * 255.) as usize).unwrap());
            output.push(u8::try_from((((max_val - pixel.depth) / diff) * 255.) as usize).unwrap());
            output.push(u8::try_from((((max_val - pixel.depth) / diff) * 255.) as usize).unwrap());
        });

        output
    }
}
