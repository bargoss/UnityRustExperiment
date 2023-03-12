use super::*;
use super::verlet_physics_world::Id;

#[derive(Debug, Clone, Copy)]
pub struct VerletBeam {
    pub verlet_object_id_a: Id,
    pub verlet_object_id_b: Id,
    pub length: FixedPoint,
}