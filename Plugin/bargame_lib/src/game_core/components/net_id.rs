use crate::game_core::common::id::Id;

use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NetId {
    pub value: Id,
}
