use super::*;


#[derive(Debug, Clone, Copy)]
pub struct VerletObject {
    pub position: FP2,
    pub position_last: FP2,
    pub acceleration: FP2,
    pub radius: FP,
    pub mass: FP,
    pub is_static: bool,
}