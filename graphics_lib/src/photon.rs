use crate::color::Color;
use glam::DVec3;
use kd_tree::KdPoint;

#[derive(Debug, Clone, Copy)]
pub enum PhotonType {
    Shadow,
    Direct,
    Indirect(Color),
    Caustic(Color),
}

#[derive(Debug, Clone)]
pub struct Photon {
    pos: [f64; 3],
    p_type: PhotonType,
    light_index: usize,
    obj: usize,
}

impl Default for Photon {
    fn default() -> Self {
        panic!()
    }
}

impl Photon {
    pub fn new_shadow(pos: DVec3, light_index: usize, obj: usize) -> Self {
        Self {
            pos: pos.to_array(),
            p_type: PhotonType::Shadow,
            light_index,
            obj,
        }
    }

    pub fn new_direct(pos: DVec3, light_index: usize, obj: usize) -> Self {
        Self {
            pos: pos.to_array(),
            p_type: PhotonType::Direct,
            light_index,
            obj,
        }
    }

    pub fn new_indirect(pos: DVec3, light_index: usize, color: Color, obj: usize) -> Self {
        Self {
            pos: pos.to_array(),
            p_type: PhotonType::Indirect(color),
            light_index,
            obj,
        }
    }

    pub fn new_caustic(pos: &DVec3, light_index: usize, color: Color, obj: usize) -> Self {
        Self {
            pos: pos.to_array(),
            p_type: PhotonType::Caustic(color),
            light_index,
            obj,
        }
    }

    pub fn is_shadow(&self) -> bool {
        matches!(self.p_type, PhotonType::Shadow)
    }

    pub fn is_direct(&self) -> bool {
        matches!(self.p_type, PhotonType::Direct)
    }

    pub fn get_light_index(&self) -> usize {
        self.light_index
    }

    pub fn get_type(&self) -> PhotonType {
        self.p_type
    }

    pub fn get_pos(&self) -> DVec3 {
        DVec3::from_array(self.pos)
    }

    pub fn get_obj(&self) -> usize {
        self.obj
    }
}

impl KdPoint for Photon {
    type Scalar = f64;
    type Dim = typenum::U3;

    fn at(&self, i: usize) -> Self::Scalar {
        self.pos[i]
    }
}
