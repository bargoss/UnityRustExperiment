use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;

pub struct GameWorld{
    pub world: World,
    pub init_schedule: Schedule,
    pub update_schedule: Schedule,
}

impl GameWorld{
    pub fn new() -> GameWorld{
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        GameWorld{
            world,
            init_schedule,
            update_schedule,
        }
    }
}