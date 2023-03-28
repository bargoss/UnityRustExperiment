use bevy_ecs::prelude::*;
use crate::arena_fight_game::bundles::*;
use crate::game_core::components::*;
use crate::arena_fight_game::components::*;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::view_components::SphereView;

fn unit_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut UnitSpawner, &Position, &BelongsToFaction, &NetId)>,
    mut net_id_counter: ResMut<NetIdCounter>,
) {
    for (mut unit_spawner, position, belongs_to_faction, net_id) in query.iter_mut() {
        let time_fixed_point = time.fixed_delta_time * time.tick;
        let time_since_last_spawn = time_fixed_point - unit_spawner.last_spawn_time;

        if time_since_last_spawn >= unit_spawner.spawn_interval {
            // Update last_spawn_time to the current time
            unit_spawner.last_spawn_time = time_fixed_point;

            // Spawn a new unit
            let unit_net_id = net_id_counter.next();
            let _ = commands.spawn(UnitBundle {
                net_id: NetId { value: unit_net_id },
                unit: Unit {
                    target_movement_position: position.value + FixedPointV2::new(FixedPoint::new(0.1), FixedPoint::new(0.0))
                },
                character_movement: CharacterMovement::default(),
                position: *position,
                rigidbody: Rigidbody {
                    velocity: FixedPointV2::default(),
                    mass: FixedPoint::new(1.0),
                },
                impulse: Impulse::default(),
                collider: CircleCollider {
                    radius: FixedPoint::new(0.5),
                },
                health: Health {
                    health: FixedPoint::new(5.0),
                    max_health: FixedPoint::new(5.0),
                    health_regen_per_second: FixedPoint::new(-0.025),
                },
                sphere_view: SphereView {
                    radius: FixedPoint::new(0.5),
                    view_custom_id: unit_net_id,
                },
                belongs_to_faction: *belongs_to_faction,
            }).id();
        }
    }
}