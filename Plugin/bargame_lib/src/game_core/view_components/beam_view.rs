use crate::game_core::components::net_id::NetId;
use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct LineView {
    pub thickness : FixedPoint,
    pub start : Id,
    pub end : Id,
}