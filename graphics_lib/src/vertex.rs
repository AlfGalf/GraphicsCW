use crate::transform::Transform;
use crate::vector::Vector;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Vertex(pub f32, pub f32, pub f32);

impl Vertex {
    pub fn apply_transform(&self, tr: &Transform) -> Self {
        let x = self.0;
        let y = self.1;
        let z = self.2;

        Self {
            0: tr.get(0, 0) * x + tr.get(1, 0) * y + tr.get(2, 0) * z + tr.get(3, 0),
            1: tr.get(0, 1) * x + tr.get(1, 1) * y + tr.get(2, 1) * z + tr.get(3, 1),
            2: tr.get(0, 2) * x + tr.get(1, 2) * y + tr.get(2, 2) * z + tr.get(3, 2),
        }
    }

    pub fn to(&self, other: &Self) -> Vector {
        Vector {
            x: other.0 - self.0,
            y: other.1 - self.1,
            z: other.2 - self.2,
        }
    }
}

impl Add<Vector> for Vertex {
    type Output = Vector;
    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.0 + other.x,
            y: self.1 + other.y,
            z: self.2 + other.z,
        }
    }
}
