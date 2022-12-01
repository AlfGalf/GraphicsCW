use glam::Vec3;
use rand::Rng;
use std::f32::consts::PI;

const NUMBER_POINTS: usize = 10_000_000;

// Based on https://stackoverflow.com/questions/19671845/how-can-i-generate-a-random-number-within-a-range-in-rust
pub fn fibonacci_spiral_random() -> Vec3 {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..NUMBER_POINTS);

    let phi = PI * (3. - 5.0_f32.sqrt());

    let y = 1. - (i as f32 / (NUMBER_POINTS - 1) as f32) * 2.; // y goes from 1 to -1
    let radius = (1. - y * y).sqrt(); // radius at y

    let theta = phi * (i as f32); // golden angle increment

    let x = theta.cos() * radius;
    let z = theta.sin() * radius;

    Vec3::new(x, y, z)
}
