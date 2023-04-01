
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct InterpolatedPosition {
    pub custom_identifier: u32,
}