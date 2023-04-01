
use bevy_ecs::prelude::*;
use crate::game_core::math::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Rigidbody {
    pub velocity: FP2,
    pub mass: FP,
}

// implement default
impl Default for Rigidbody {
    fn default() -> Self {
        Rigidbody {
            velocity: FP2::default(),
            mass: FP::new(1.0),
        }
    }
}