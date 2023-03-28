use crate::game_core::common::id::Id;
use super::*;
use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct NetId {
    pub value: Id,
}
