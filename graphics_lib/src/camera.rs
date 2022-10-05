use crate::ray::Ray;
use crate::vector::Vector;
use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Camera {
    pub position: Vertex,
    pub direction: Vector,
    pub focal_length: f32,
}

impl Camera {
    // x should vary -1 -> 1
    // y should vary -a -> a
    pub fn ray(&self, x: f32, y: f32) -> Ray {
        let up = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let right = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let dir = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        Ray {
            position: self.position.clone(),
            direction: (&dir * self.focal_length)
                + (&up * (y as f32 / 2.))
                + (&right * (x as f32 / 2.)),
        }
    }
}
