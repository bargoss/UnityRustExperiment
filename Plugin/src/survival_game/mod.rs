use std::collections::HashMap;
use bevy_ecs;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, Query, Res, ResMut, Schedule, SystemStage};
use bevy_ecs::schedule::Stage;
use bevy_ecs::system::CommandQueue;
use bevy_ecs::world::World;
use bevy_math::Vec3;

use crate::bubbles::spatial_ds::LookUpGrids;

mod static_world;
mod structs;
mod components;
//use crate::survival_game::static_world::terrain::Terrain;


pub struct Game {
    world: World,
    update_schedule: Schedule,
    next_id: u32,
}
impl Game {
    pub fn new() -> Game {
        let mut world = World::new();

        let terrain = static_world::tilemap::TileMap::new(256, 256);
        world.insert_resource(terrain);

        let mut update_schedule = Schedule::default();

        let pre_update_stage = SystemStage::single_threaded();
        //pre_update_stage.add_system(update_lookup_grids);
        //pre_update_stage.add_system(update_external_id_res);
        update_schedule.add_stage("pre_update", pre_update_stage);

        let update_stage = SystemStage::single_threaded();
        update_schedule.add_stage("update", update_stage);

        Game {
            world,
            update_schedule,
            next_id: 0,
        }
    }
}


// create tests for this:

#[cfg(test)]
mod tests {
    
    #[test]
    fn main_test(){
        assert_eq!(2 + 2, 4);
    }
}
