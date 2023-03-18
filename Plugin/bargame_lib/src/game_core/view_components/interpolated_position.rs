use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct InterpolatedPosition {
    pub value: FixedPointV2
}