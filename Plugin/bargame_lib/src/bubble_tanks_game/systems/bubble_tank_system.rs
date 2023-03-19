use bevy_ecs::prelude::*;
use crate::bubble_tanks_game::BubbleTanksInput;
use crate::bubble_tanks_game::components::tank_bubble::TankBubble;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::game_world::PlayerInputMap;

pub fn bubble_tank_system(
    query: Query<(Entity, &TankBubble, &Position, &Rigidbody)>,
    player_input_map: Res<PlayerInputMap<BubbleTanksInput>>,
){
    todo!();
}