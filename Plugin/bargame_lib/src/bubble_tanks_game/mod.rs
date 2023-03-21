use std::collections::HashMap;
use bevy_ecs::prelude::SystemStage;
use crate::game_core::game_world::GameWorld;
use crate::game_core::math::{FixedPoint, FixedPointV2};
use crate::game_core::systems::physics_system::{pull_bodies, push_all_bodies, run_physics_step};
use crate::game_core::view_components::Id;
use crate::rollback_controller::input::Input;

mod systems;
mod components;



#[derive(Copy, Clone, Debug, Default)]
pub struct BubbleTanksInput{
    pub movement_dir : FixedPointV2,
    pub steer : FixedPoint,
    pub fire : bool,
}

impl Input for BubbleTanksInput {

}

pub struct BubbleTanksGame{
    game_core: GameWorld<BubbleTanksInput>,
}

impl BubbleTanksGame {
    pub fn new(fixed_delta_time: FixedPoint) -> Self {
        let mut game_core = GameWorld::new(fixed_delta_time);
        game_core.add_stage_to_advance_tick_schedule("physics", SystemStage::single_threaded()
            .with_system(push_all_bodies)
            .with_system(run_physics_step)
            .with_system(pull_bodies)
        );

        //game_core.add_stage_to_advance_tick_schedule("update", SystemStage::single_threaded()
            //.with_system(systems::bubble_tank_system::bubble_tank_system));

        Self {
            game_core,
        }
    }

    pub fn advance_tick(&mut self, input_map: HashMap<Id, BubbleTanksInput>){ self.game_core.advance_tick(input_map); }
    pub fn register_keyframes(&mut self){ self.game_core.register_keyframes(); }
    pub fn sample_view_snapshots(&mut self, viewing_time: f64){ self.game_core.sample_view_snapshots(viewing_time); }
}
