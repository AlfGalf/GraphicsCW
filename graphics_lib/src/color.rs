use glam::Vec3;
use std::iter::Sum;
use std::ops::{Add, Mul};

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

    pub fn new_black() -> Self {
        Self { color: Vec3::ZERO }
    }

    pub fn red(&self) -> f32 {
        self.color.x
    }
    pub fn green(&self) -> f32 {
        self.color.y
    }
    pub fn blue(&self) -> f32 {
        self.color.z
    }

    pub fn scale(&self, rhs: &Self) -> Self {
        Self {
            color: self.color * rhs.color,
        }
    }
}

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
