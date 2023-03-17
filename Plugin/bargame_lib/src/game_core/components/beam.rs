use crate::game_core::common::id::Id;
use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Beam {
    pub a: Id,
    pub b: Id,
    pub length: FixedPoint,
}