use crate::game_core::components::net_id::NetId;
use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Beam {
    pub a: NetId,
    pub b: NetId,
    pub length: FixedPoint,
}