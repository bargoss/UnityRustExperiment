use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::*;
use crate::game_core::common::Id;
use crate::game_core::components::*;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;

pub fn test_system(
    mut query: Query<(&mut Position)>,
    id_entity_map: Res<IdEntityMap>,
) {
    let res = id_entity_map.get_mut_from_query(&mut query, Id::new(0));
}
/*
pub fn unit_attack_system(
    mut attacking_unit_query: Query<(&mut Unit, &Position, &BelongsToFaction)>,
    mut target_unit_query: Query<(&Unit, &Position, &mut Impulse, &mut Health, &BelongsToFaction)>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
    time: Res<Time>,
){
    let unit_attack_dps = 5;
    let unit_damage = time.fixed_delta_time * unit_attack_dps;
    let unit_attack_range = FixedPoint::new(0.5);
    let mut nearby_bodies_query_buffer = Vec::new();

    for (mut attacking_unit, attacking_unit_position, attacking_unit_faction) in attacking_unit_query.iter_mut() {
        physics_world.overlap_circle(attacking_unit_position.value, FixedPoint::new(0.5), &mut nearby_bodies_query_buffer);

        for body_id in nearby_bodies_query_buffer.iter() {
            let res = id_entity_map.get_mut_from_query(&mut target_unit_query, Id::new(*body_id));
            match res {
                Some((target_unit, target_unit_position, mut target_unit_impulse, mut target_unit_health, target_unit_faction)) => {
                    if attacking_unit_faction.faction != target_unit_faction.faction {
                        let distance = (attacking_unit_position.value - target_unit_position.value).magnitude_squared();
                        if distance < unit_attack_range * unit_attack_range {
                            let push = (target_unit_position.value - attacking_unit_position.value).normalize() * FixedPoint::new(0.1);
                            target_unit_impulse.value += push;
                            target_unit_health.health -= unit_damage;
                        }
                    }
                },
                None => {}
            }
        }
    }

}
*/