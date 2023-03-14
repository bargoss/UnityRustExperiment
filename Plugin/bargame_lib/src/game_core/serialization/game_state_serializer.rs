use std::error::Error;
use bevy_ecs::world::World;

pub fn serialize_game_state(
    world: &World,
    game_state: &GameState,
    snapshot: &mut Snapshot,
) -> Result<(), Error> {
    let mut game_state_serializer = GameStateSerializer::new(world, game_state, snapshot);
    game_state_serializer.serialize_game_state()
}