use std::collections::HashMap;
use crate::game_core::game_world::GameWorld;
use crate::game_core::view_components::{FixedPoint, Id};
use crate::rollback_controller::input::Input;
//use bevy_ecs::schedule::IntoSystemConfigs;
use crate::arena_fight_game::components::{Character, Health, PlayerControl};
use crate::game_core::components::circle_collider::CircleCollider;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;
use bevy_ecs::prelude::*;
use crate::arena_fight_game::systems::character_movement_system::character_movement;
use crate::game_core::components::impulse::Impulse;

#[derive(Copy, Clone, Debug, Default)]
pub struct ArenaFightInput{}
unsafe impl Sync for ArenaFightInput {}
unsafe impl Send for ArenaFightInput {}
impl Input for ArenaFightInput {}

#[derive(Bundle)]
pub struct PlayerCharacterBundle {
    pub position: Position,
    pub rigidbody: Rigidbody,
    pub impulse: Impulse,
    pub collider: CircleCollider,
    pub health: Health,
    pub player_control: PlayerControl,
    pub character: Character,
}

pub struct ArenaFightGame {
    pub game_world: GameWorld<ArenaFightInput>,
}

pub fn dummy_system() {
    println!("dummy system arena fight");
}

impl ArenaFightGame {
    pub fn new() -> Self {
        let mut game_world = GameWorld::new(FixedPoint::new(0.02) ,(dummy_system,character_movement, ).chain());
        Self {
            game_world,
        }
    }

    pub fn advance_tick(&mut self, input_map: HashMap<Id, ArenaFightInput>){ self.game_world.advance_tick(input_map); }
    pub fn register_keyframes(&mut self){ self.game_world.register_keyframes(); }
    pub fn sample_view_snapshots<T>(&mut self, viewing_time: f64, buffer: &mut Vec<T>) where T: ViewSnapshot + 'static { self.game_world.sample_view_snapshots(viewing_time, buffer); }
}
