use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub value: FixedPointV2,
    pub impulse: FixedPointV2
}