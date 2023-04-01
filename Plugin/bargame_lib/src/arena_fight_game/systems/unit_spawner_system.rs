use bevy_ecs::prelude::*;
use crate::arena_fight_game::bundles::*;
use crate::game_core::components::*;
use crate::arena_fight_game::components::*;
use crate::game_core::common::{Id, IdGenerator, IdGeneratorBuilder, Random};
use crate::game_core::math::*;
use crate::game_core::resources::*;
pub fn unit_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut UnitSpawner, &Position, &BelongsToFaction, &NetId, &CircleCollider)>,
) {
    let mut new_units = Vec::new();

    for (mut unit_spawner, position, belongs_to_faction, net_id, circle_collider) in query.iter_mut() {
        let time_fixed_point = time.fixed_delta_time * time.tick;
        let time_since_last_spawn = time_fixed_point - unit_spawner.last_spawn_time;

        if time_since_last_spawn >= unit_spawner.spawn_interval {
            // Update last_spawn_time to the current time
            unit_spawner.last_spawn_time = time_fixed_point;

            let spawn_offset = Random::seed_fixed_point(time.fixed_delta_time +  position.value.y() + position.value.x())
                .next_fixed_point_on_unit_circle() * circle_collider.radius;

            let unit_spawn_position = position.value + spawn_offset;
            new_units.push(create_unit_creation_command(*net_id, unit_spawn_position, belongs_to_faction.faction));
        }
    }

    // sort by creator net_id and execute
    new_units.sort_by(|a, b| a.creators_net_id.value.cmp(&b.creators_net_id.value));
    for unit_creation_command in new_units {
        let net_id = NetId::from_id(IdGeneratorBuilder::start().hash_u32(time.tick).hash_u32(562313).hash_net_id(unit_creation_command.creators_net_id).finish());
        execute_unit_creation_command(
            unit_creation_command,
            net_id,
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
    bundle.unit_view.view_custom_id = net_id.value;
    bundle.unit.following_entity = Some(unit_creation_command.creators_net_id);

    commands.spawn(bundle);
}

fn create_unit_creation_command(
    creators_net_id : NetId,
    position: FP2,
    faction: Faction
) -> UnitCreationCommand
{
    let bundle = UnitBundle {
        net_id: NetId { value: Id::new(0) },
        unit: Unit {
            following_entity: None
        },
        character_movement: CharacterMovement::default(),
        position: Position{value: position},
        rigidbody: Rigidbody {
            velocity: FP2::default(),
            mass: FP::new(1.0),
        },
        impulse: Impulse::default(),
        collider: CircleCollider {
            radius: FP::new(0.25),
        },
        health: Health {
            health: FP::new(1.0),
            max_health: FP::new(1.0),
            health_regen_per_second: FP::new(-0.05),
        },
        unit_view: UnitView {
            view_custom_id: Id::new(0),
        },
        belongs_to_faction: BelongsToFaction{faction},
    };

    UnitCreationCommand {
        creators_net_id,
        unit_bundle: bundle,
    }
}