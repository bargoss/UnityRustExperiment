use super::*;


#[derive(Debug, Clone, Copy)]
pub struct VerletObject {
    pub position: FixedPointV2,
    pub position_last: FixedPointV2,
    pub acceleration: FixedPointV2,
    pub radius: FixedPoint,
    pub is_static: bool,
}