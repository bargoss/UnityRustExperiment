use super::*;
use bevy_ecs::prelude::*;

pub fn run_system_once<Params>(world: &mut World, system: impl bevy_ecs::schedule::IntoSystemDescriptor<Params>) {
    SystemStage::single_threaded().add_system(system).run(world);
}