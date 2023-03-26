use crate::game_core::components::*;
use bevy_ecs::prelude::*;

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
    use crate::game_core::common::*;
    use crate::game_core::math::*;
    use super::*;

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