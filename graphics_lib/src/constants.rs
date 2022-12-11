// Small number to allow for error in calculations
pub(crate) const EPSILON: f32 = 1.0E-3;
// Minimum coefficient for a color calculation before its ignored
pub(crate) const MIN_RECURSE_COEFFICIENT: f32 = 1.0E-8;
// Maximum number of recursions for a color calculation before its ignored
pub(crate) const MAX_RECURSE_DEPTH: usize = 10;
// Maximum number of recursions for a photon calculation before its ignored
pub(crate) const MAX_PHOTON_RECURSE_DEPTH: usize = 6;
// Default ambient coefficient for objects
pub(crate) const DEFAULT_AMBIENT: f32 = 0.01;
// Scene bounds, this is needed for the bounds of primitives as they cannot
//  use -Infinity or +Infinity as this breaks the BVH
pub(crate) const SCENE_BOUNDS: f32 = 1.0E10;
// How many photons and caustic photons to fire
pub(crate) const NUMBER_PHOTONS_PER_LIGHT: usize = 300_000;
pub(crate) const NUMBER_CAUSTICS_PER_LIGHT_PER_OBJ: usize = 100_000;
// Radius to find photons in when estimating radiance
pub(crate) const PHOTON_RAD: f32 = 0.8;
// Radius to find caustics in when estimating caustic effect
pub(crate) const CAUSTIC_RAD: f32 = 0.05;
