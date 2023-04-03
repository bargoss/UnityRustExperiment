use super::super::data_types::*;
use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Unit {
    pub velocity: Vec2FFloat
}

