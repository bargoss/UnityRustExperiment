use bevy_ecs::prelude::*;
use crate::game_core::components::depend_on_2_entities::DependOnRel2;
use crate::game_core::resources::id_entity_map::IdEntityMap;

pub fn depend_on_rel_system(
    depend_on_rel_2_query: Query<(Entity, &DependOnRel2)>,
    entity_query: Query<(Entity)>,
    id_entity_map: Res<IdEntityMap>,
    commands: &mut Commands,
){
    for (entity, depend_on_rel_2) in depend_on_rel_2_query.iter(){

        let mut rel_0_exists = match id_entity_map.get(depend_on_rel_2.net_id_0) {
            Some(entity) => entity_query.get(entity).is_ok(),
            None => false,
        };
        let mut rel_1_exists = match id_entity_map.get(depend_on_rel_2.net_id_1) {
            Some(entity) => entity_query.get(entity).is_ok(),
            None => false,
        };

        if !rel_0_exists || !rel_1_exists {
            commands.entity(entity).despawn();
        }
    }
}


#[test]
fn depend_on_rel_system_test() {
    todo!() //gotta test this
}
