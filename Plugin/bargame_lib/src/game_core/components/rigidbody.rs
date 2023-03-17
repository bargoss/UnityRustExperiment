use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Rigidbody {
    pub velocity: FixedPointV2,
    pub impulse: FixedPointV2,
    pub mass: FixedPoint,
}