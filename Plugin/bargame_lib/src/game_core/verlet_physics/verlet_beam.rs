use super::*;

#[derive(Debug, Clone, Copy)]
pub struct VerletBeam {
    pub verlet_object_id_a: u32,
    pub verlet_object_id_b: u32,
    pub length: FixedPoint,
}