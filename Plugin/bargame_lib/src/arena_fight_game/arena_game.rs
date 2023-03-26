use std::collections::HashMap;
use crate::game_core::game_world::GameWorld;
use crate::game_core::view_components::{FixedPoint, Id};
use crate::rollback_controller::input::Input;
//use bevy_ecs::schedule::IntoSystemConfigs;
use crate::arena_fight_game::components::{Character, Health, PlayerCharacterControl};
use crate::game_core::components::circle_collider::CircleCollider;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;
use bevy_ecs::prelude::*;
use crate::arena_fight_game::systems::character_movement_system::character_movement;
use crate::arena_fight_game::systems::player_input_system::player_input_system;
use crate::game_core::components::impulse::Impulse;
use crate::game_core::math::FixedPointV2;
use crate::game_core::view_components::sphere_view::SphereView;

#[derive(Copy, Clone, Debug, Default)]
pub struct ArenaFightInput{
    pub movement_direction: FixedPointV2,
}
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
    pub player_control: PlayerCharacterControl,
    pub character: Character,
    pub sphere_view: SphereView,
}

pub struct ArenaFightGame {
    pub game_world: GameWorld<ArenaFightInput>,
}

impl ArenaFightGame {
    pub fn new() -> Self {
        let mut game_world = GameWorld::new(
            FixedPoint::new(0.02) ,
            (
                player_input_system,
                character_movement
            ).chain()
        );
        Self {
            game_world,
        }
    }

    pub fn add_player_character(&mut self, id: Id, position: FixedPointV2) {
        // default bundle
        self.game_world.world.spawn(PlayerCharacterBundle {
            position: Position{ value: position, },
            rigidbody: Rigidbody::default(),
            impulse: Impulse::default(),
            collider: CircleCollider { radius: FixedPoint::new(0.5), },
            health: Health{health: FixedPoint::new(100.0), max_health: FixedPoint::new(100.0), health_regen_per_second: FixedPoint::new(10.0),},
            player_control: PlayerCharacterControl{controlling_player_id: id},
            character: Character::default(),
            sphere_view: SphereView{view_custom_id: Id::new(0), radius: FixedPoint::new(0.5),},
        });
    }

    pub fn advance_tick(&mut self, input_map: HashMap<Id, ArenaFightInput>){ self.game_world.advance_tick(input_map); }
    pub fn register_keyframes(&mut self){ self.game_world.register_keyframes(); }
    pub fn sample_view_snapshots<T>(&mut self, viewing_time: f64, buffer: &mut Vec<T>) where T: ViewSnapshot + 'static { self.game_world.sample_view_snapshots(viewing_time, buffer); }
}
