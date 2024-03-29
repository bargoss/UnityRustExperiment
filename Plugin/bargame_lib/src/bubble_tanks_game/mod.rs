use std::collections::HashMap;
use crate::game_core::game_world::GameWorld;
use crate::game_core::math::{FixedPoint, FixedPointV2};


use crate::game_core::common::*;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

use bevy_ecs::schedule::IntoSystemConfigs;
use crate::game_core::input::Input;

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
pub fn dummy_system() {
    println!("dummy system bubble tanks");
}

impl BubbleTanksGame {
    pub fn new(fixed_delta_time: FixedPoint) -> Self {
        let game_core = GameWorld::new(fixed_delta_time, (dummy_system,).chain(), (dummy_system,).chain());

        //game_core.add_stage_to_advance_tick_schedule("update", SystemStage::single_threaded()
            //.with_system(systems::bubble_tank_system::bubble_tank_system));
        Self {
            game_core,
        }
    }

    pub fn advance_tick(&mut self, input_map: HashMap<Id, BubbleTanksInput>){ self.game_core.advance_tick(input_map); }
    pub fn register_keyframes(&mut self){ self.game_core.register_keyframes(); }
    pub fn sample_view_snapshots<T>(&mut self, viewing_time: FixedPoint, buffer: &mut Vec<T>)
        where T: ViewSnapshot + 'static
    {
        self.game_core.sample_view_snapshots(viewing_time, buffer);
    }
}
