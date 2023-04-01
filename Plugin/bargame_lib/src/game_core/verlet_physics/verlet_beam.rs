use crate::game_core::common::id::Id;
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct VerletBeam {
    pub verlet_object_id_a: Id,
    pub verlet_object_id_b: Id,
    pub length: FP,
}