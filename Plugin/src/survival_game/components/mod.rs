use bevy_ecs::prelude::Component;
use bevy_math::Vec3;

use super::static_world::tilemap::TileMapColliderId;



#[derive(Clone, Debug)]
pub struct PawnId(u32);

#[derive(Component, Clone, Debug)]
pub struct Pawn{
    pub id: u32,
    pub position: Vec3,
}

#[derive(Component, Clone, Debug)]
pub struct DynamicCollider{
    pub radius: f32,
}

#[derive(Component, Clone, Debug)]
pub struct Health{
    pub health: u32,
    pub health_regen: i32,
    pub max_health: u32,
}

#[derive(Component, Clone, Debug)]
pub struct Energy{
    pub energy: u32,
    pub energy_regen: i32,
    pub max_energy: u32,
}

#[derive(Component, Clone, Debug)]
pub struct PawnInventory {
    pub container_id: u32,
}

#[derive(Component, Clone, Debug)]
pub struct Building{
    pub id: u32,
}

#[derive(Component, Clone, Debug)]
pub struct TileMapCollider{
    pub id: TileMapColliderId,
}





