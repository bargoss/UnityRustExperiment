use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::*;
use crate::game_core::common::Id;
use crate::game_core::components::*;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;


pub fn unit_attack_system(
    mut unit_query: Query<(&mut Unit, &Position, &mut Impulse, &mut Health, &BelongsToFaction, &NetId)>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
    time: Res<Time>,
) {
    let unit_attack_dps = FP::new(0.1); // 5
    let unit_damage = time.fixed_delta_time * unit_attack_dps;
    let unit_attack_range = FP::new(0.5);
    let mut nearby_bodies_query_buffer = Vec::new();
    let mut actions = Vec::new();

    for (_attacking_unit, attacking_unit_position, _attacking_unit_impulse, _attacking_unit_health, attacking_unit_faction, attacking_unit_net_id) in unit_query.iter() {
        physics_world.overlap_circle(attacking_unit_position.value, FP::new(0.5), &mut nearby_bodies_query_buffer);

        // exclude self
        for body_id in nearby_bodies_query_buffer.iter() {
            if *body_id == attacking_unit_net_id.value.0 {
                continue;
            }
            let res = id_entity_map.get_from_query(&unit_query, Id::new(*body_id));
            if let Some((_target_unit, target_unit_position, _target_unit_impulse, _target_unit_health, target_unit_faction, target_unit_net_id)) = res {
                if attacking_unit_faction.faction != target_unit_faction.faction {
                    let distance = (attacking_unit_position.value - target_unit_position.value).magnitude_squared();
                    if distance < unit_attack_range * unit_attack_range {
                        let push = (target_unit_position.value - attacking_unit_position.value).normalize() * FP::new(0.1);
                        actions.push((target_unit_net_id.value.0, push, unit_damage));
                    }
                }
            }
        }
    }

    // Apply buffered actions
    for (target_unit_id, push, damage) in actions {
        let res = id_entity_map.get_mut_from_query(&mut unit_query, Id::new(target_unit_id));
        if let Some((_target_unit, _target_unit_position, mut target_unit_impulse, mut target_unit_health, _target_unit_faction, _target_unit_net_id)) = res {
            target_unit_impulse.value += push;
            target_unit_health.health -= damage;
        }
    }
}





