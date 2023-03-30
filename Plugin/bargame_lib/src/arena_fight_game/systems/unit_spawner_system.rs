use bevy_ecs::prelude::*;
use crate::arena_fight_game::bundles::*;
use crate::game_core::components::*;
use crate::arena_fight_game::components::*;
use crate::game_core::common::Id;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::view_components::SphereView;
pub fn unit_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut UnitSpawner, &Position, &BelongsToFaction, &NetId)>,
    mut net_id_counter: ResMut<NetIdCounter>,
) {
    let mut new_units = Vec::new();

    for (mut unit_spawner, position, belongs_to_faction, net_id) in query.iter_mut() {
        let time_fixed_point = time.fixed_delta_time * time.tick;
        let time_since_last_spawn = time_fixed_point - unit_spawner.last_spawn_time;

        if time_since_last_spawn >= unit_spawner.spawn_interval {
            // Update last_spawn_time to the current time
            unit_spawner.last_spawn_time = time_fixed_point;

            let unit_spawn_position = position.value + FixedPointV2::new(FixedPoint::new(0.1), FixedPoint::new(0.0));
            new_units.push(create_unit_creation_command(*net_id, unit_spawn_position, belongs_to_faction.faction));
        }
    }

    // sort by creator net_id and execute
    new_units.sort_by(|a, b| a.creators_net_id.value.cmp(&b.creators_net_id.value));
    for unit_creation_command in new_units {
        execute_unit_creation_command(
            unit_creation_command,
             NetId{value: net_id_counter.next() },
            &mut commands
        );
    }
}

struct UnitCreationCommand {
    creators_net_id: NetId,
    unit_bundle: UnitBundle,
}
fn execute_unit_creation_command(
    unit_creation_command: UnitCreationCommand,
    net_id: NetId,
    commands: &mut Commands,
){
    let mut bundle = unit_creation_command.unit_bundle;

    bundle.net_id = net_id;
    bundle.sphere_view.view_custom_id = net_id.value;

    commands.spawn(bundle);
}

fn create_unit_creation_command(
    creators_net_id : NetId,
    position: FixedPointV2,
    faction: Faction
) -> UnitCreationCommand
{
    let bundle = UnitBundle {
        net_id: NetId { value: Id::new(0) },
        unit: Unit {
            target_movement_position: position,
        },
        character_movement: CharacterMovement::default(),
        position: Position{value: position},
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
            view_custom_id: Id::new(0),
        },
        belongs_to_faction: BelongsToFaction{faction},
    };

    UnitCreationCommand {
        creators_net_id,
        unit_bundle: bundle,
    }
}