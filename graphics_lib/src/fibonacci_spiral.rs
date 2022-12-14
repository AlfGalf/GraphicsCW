use glam::DVec3;
use rand::Rng;
use std::f64::consts::PI;

const NUMBER_POINTS: usize = 100_000_000;

// Based on https://stackoverflow.com/questions/19671845/how-can-i-generate-a-random-number-within-a-range-in-rust
// Generates a vector in a "random" direction
pub(crate) fn fibonacci_spiral_random() -> DVec3 {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..NUMBER_POINTS);

    let phi = PI * (3. - 5.0_f64.sqrt());

    let y = 1. - (i as f64 / (NUMBER_POINTS - 1) as f64) * 2.; // y goes from 1 to -1
    let radius = (1. - y * y).sqrt(); // radius at y

    let theta = phi * (i as f64); // golden angle increment

    let x = theta.cos() * radius;
    let z = theta.sin() * radius;

    DVec3::new(x, y, z)
}

// Generates a vector in a "random" direction in a hemisphere centered around a vector
pub(crate) fn hemisphere_random(dir: DVec3) -> DVec3 {
    let rand_dir = fibonacci_spiral_random();

    if dir.dot(rand_dir) < 0. {
        -rand_dir
    } else {
        rand_dir
    }
}
