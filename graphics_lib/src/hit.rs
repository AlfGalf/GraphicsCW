use glam::DVec3;

// Represents a hit between a ray and a primitive
#[derive(Clone)]
pub struct Hit {
    // The location of the hit
    pos: DVec3,
    // The normal of the surface at the hit
    normal: DVec3,
    // The distance from the ray origin to the hit
    distance: f64,
    // Index of the object the primitive hit came from
    obj_index: usize,
    // true if this hit was entering the primitive, false otherwise
    correct_dir: bool,
    // CSG of the leaf the object hit in the CSG tree
    csg_index: usize,
}

impl Hit {
    pub(crate) fn new(
        pos: DVec3,
        normal: DVec3,
        distance: f64,
        correct_dir: bool,
        obj_index: usize,
        csg_index: usize,
    ) -> Hit {
        Hit {
            pos,
            normal: normal.normalize(),
            distance,
            correct_dir,
            obj_index,
            csg_index,
        }
    }

    pub(crate) fn normal(&self) -> &DVec3 {
        &self.normal
    }

    pub(crate) fn pos(&self) -> &DVec3 {
        &self.pos
    }

    pub(crate) fn get_object_index(&self) -> usize {
        self.obj_index
    }

    pub(crate) fn get_distance(&self) -> f64 {
        self.distance
    }

    pub(crate) fn get_dir(&self) -> bool {
        self.correct_dir
    }

    pub(crate) fn get_csg_index(&self) -> usize {
        self.csg_index
    }

    // Reverses the direction of a hit
    // Used by CSG's to modify hits entering a object to instead be exiting
    pub(crate) fn flip(&mut self) {
        self.correct_dir = !self.correct_dir;
        self.normal = -self.normal;
    }
}
