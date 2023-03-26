use crate::game_core::common::*;
use bevy_ecs::component::Component;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct DependOnRel2{
    pub net_id_0: Id,
    pub net_id_1: Id,
}
