use bevy_ecs::prelude::*;
use crate::game_core::components::net_id::NetId;
use crate::game_core::resources::id_entity_map::IdEntityMap;

pub fn id_entity_map_sync_system(
    net_id_query: Query<(Entity, &NetId)>,
    mut id_entity_map: ResMut<IdEntityMap>
){
    id_entity_map.clear();
    for (entity, net_id) in net_id_query.iter(){
        id_entity_map.insert(net_id.value, entity);
    }
}