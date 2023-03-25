use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Impulse {
    pub value: FixedPointV2,
}