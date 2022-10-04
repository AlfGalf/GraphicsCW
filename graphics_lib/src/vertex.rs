use crate::transform::Transform;

#[derive(Debug)]
pub struct Vertex(pub f32, pub f32, pub f32);

impl Vertex {
    pub fn apply_transform(&mut self, tr: &Transform) -> &mut Self {
        self.0 =
            tr.get(0, 0) * self.0 + tr.get(1, 0) * self.1 + tr.get(2, 0) * self.2 + tr.get(3, 0);
        self.1 =
            tr.get(0, 1) * self.0 + tr.get(1, 1) * self.1 + tr.get(2, 1) * self.2 + tr.get(3, 1);
        self.2 =
            tr.get(0, 2) * self.0 + tr.get(1, 2) * self.1 + tr.get(2, 2) * self.2 + tr.get(3, 2);
        self
    }
}
