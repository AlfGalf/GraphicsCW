use glam::Vec3;
use kd_tree::KdPoint;

enum PhotonType {
    Shadow,
    Direct,
    Indirect,
    // Caustic,
}

pub struct Photon {
    pos: [f32; 3],
    p_type: PhotonType,
    light_index: usize,
}

impl Photon {
    pub fn new_shadow(pos: Vec3, light_index: usize) -> Self {
        Self {
            pos: pos.to_array(),
            p_type: PhotonType::Shadow,
            light_index,
        }
    }
}

impl KdPoint for Photon {
    type Scalar = f32;
    type Dim = typenum::U3;

    fn at(&self, i: usize) -> Self::Scalar {
        self.pos[i]
    }
}
