use glam::Vec3;
use std::iter::Sum;
use std::ops::{Add, Mul};

// Helper struct to represent any color values in the project
// Wrapper round the Vec3 from glam
// Allows for the color arithmetic to be altered
#[derive(Debug, Clone, Copy)]
pub struct Color {
    color: Vec3,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color: Vec3::new(red, green, blue),
        }
    }

    pub fn new_grey(c: f32) -> Self {
        Self {
            color: Vec3::new(c, c, c),
        }
    }

    pub fn new_black() -> Self {
        Self { color: Vec3::ZERO }
    }

    pub(crate) fn magnitude(&self) -> f32 {
        (self.color.x + self.color.y + self.color.z) / 3.
    }

    pub(crate) fn red(&self) -> f32 {
        self.color.x
    }
    pub(crate) fn green(&self) -> f32 {
        self.color.y
    }
    pub(crate) fn blue(&self) -> f32 {
        self.color.z
    }

    // Multiplies two colors together piecewise
    pub(crate) fn piecewise_mul(&self, rhs: &Self) -> Self {
        Self {
            color: self.color * rhs.color,
        }
    }

    // Multiplies with constant magnitude
    pub(crate) fn mul_const_mag(&self, c: &Color) -> Color {
        self.piecewise_mul(c) * (1. / c.magnitude())
    }

    pub(crate) fn min_val(&self) -> f32 {
        self.color.min_element()
    }

    pub(crate) fn max_val(&self) -> f32 {
        self.color.max_element()
    }
}

// Implements arithmetic operators for Color
impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            color: self.color + rhs.color,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            color: self.color * rhs,
        }
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::new(0., 0., 0.), |col, next| col + next)
    }
}
