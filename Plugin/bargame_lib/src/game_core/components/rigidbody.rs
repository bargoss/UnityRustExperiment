use super::*;
use bevy_ecs::prelude::*;
use crate::game_core::math::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Rigidbody {
    pub velocity: FixedPointV2,
    pub mass: FixedPoint,
}

// implement default
impl Default for Rigidbody {
    fn default() -> Self {
        Rigidbody {
            velocity: FixedPointV2::default(),
            mass: FixedPoint::new(1.0),
        }
    }
}