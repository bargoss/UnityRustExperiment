use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use crate::game_core::components::circle_collider::CircleCollider;
use super::components::*;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use bevy_ecs::bundle::Bundle;
use crate::game_core::components::net_id::NetId;

// "particle" bundle that has components Position, Velocity, Collider
#[derive(Bundle)]
pub struct ParticleBundle {
    pub position: Position,
    pub velocity: Rigidbody,
    pub collider: CircleCollider,
}

#[derive(Bundle)]
pub struct SimpleBundle {
    pub id: NetId,
}


#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;
    use crate::game_core::view_components::Id;
    use super::*;

    fn query_directly_from_world_experiment(mut world: World) {
        //let mut entities = world.query::<&NetId>().iter().collect::<Vec<_>>();
        //for entity in entities {
        //    world.despawn(entity);
        //}
    }

    // get the command queue as well
    fn destroy_and_check_system(
        query: Query<(Entity,&NetId)>,
        mut commands: Commands,
    ) {
        // vec of Entity
        let mut entities = Vec::new();
        for (entity, _net_id) in query.iter() {
            entities.push(entity);
            commands.entity(entity).despawn();
        }
        commands.spawn(SimpleBundle {
            id: NetId { value: Id::new(2) },
        });

        //// see if we can access them
        //for entity in entities {
        //    let entity_result = query.get(entity);
        //    println!("net_id: {:?}", entity_result);
        //}
    }

    fn print_system(query: Query<(Entity, &NetId)>) {
        println!("result:");
        for net_id in query.iter() {
            println!("  printing: {:?}", net_id);
        }
    }

    #[test]
    fn after_destroy_experiment() {
        let mut world = World::new();
        let mut update_schedule = Schedule::default();
        update_schedule.add_systems((
            destroy_and_check_system,
            print_system,
        ).chain());

        world.spawn(SimpleBundle {
            id: NetId { value: Id::new(0) },
        });

        world.spawn(SimpleBundle {
            id: NetId { value: Id::new(1) },
        });

        update_schedule.run(&mut world);
        update_schedule.run(&mut world);
    }

    #[test]
    fn spawn_and_mutate_experiment() {
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        // spawn bundle
        world.spawn(ParticleBundle{
            position: Position{value: FixedPointV2::from_num(0.0, 0.0)},
            velocity: Default::default(),
            collider: Default::default(),
        });

        // access the entity, add one more component
        let entity = world.spawn_empty().id();
        let mut position = world.get_mut::<Position>(entity).unwrap();
        (*position).value = FixedPointV2::from_num(1.0, 1.0);
        println!("position: {:?}", *position);

    }
}