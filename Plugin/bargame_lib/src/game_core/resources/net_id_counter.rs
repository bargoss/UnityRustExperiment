use crate::game_core::common::Id;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug)]
pub struct NetIdCounter(u32);

impl NetIdCounter {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> Id {
        let id = self.0;
        self.0 = self.0.wrapping_add(1);
        Id(id)
    }
}
