use crate::game_core::game_world::GameWorld;
use crate::game_core::view_components::FixedPoint;
use crate::rollback_controller::input::Input;
use bevy_ecs::schedule::IntoSystemConfigs;

#[derive(Copy, Clone, Debug, Default)]
pub struct ArenaFightInput{

}
unsafe impl Sync for ArenaFightInput {}
unsafe impl Send for ArenaFightInput {}
impl Input for ArenaFightInput {}

pub struct ArenaFightGame {
    pub game_world: GameWorld<ArenaFightInput>,
}

impl ArenaFightGame {
    pub fn new() -> Self {
        let mut game_world = GameWorld::new(FixedPoint::new(0.02) ,().chain());
        Self {
            game_world,
        }
    }
}
