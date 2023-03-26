use std::collections::HashMap;
use bevy_ecs::prelude::*;
use crate::bubble_tanks_game::BubbleTanksInput;
use crate::bubble_tanks_game::components::tank_bubble::TankBubble;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::game_world::PlayerInputMap;
use crate::game_core::math::{FixedPoint, FixedPointV2};
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::common::*;


// Q: how can I describe something like Query<(Entity, &TankBubble, &Position, &Rigidbody)> in a generic way?


/*
pub fn bubble_tank_system(
    query: Query<(Entity, &TankBubble, &Position, &Rigidbody)>,
    player_input_map: Res<PlayerInputMap<BubbleTanksInput>>,
    id_entity_map: Res<IdEntityMap>,
){
    //bubble_tank_to_com as HashMap<Id, FixedPointV2>
    let mut bubble_tank_to_mass_position = HashMap::new();
    let mut bubble_tank_to_body_count = HashMap::new();
    let mut bubble_tank_to_center_of_mass = HashMap::new();
    for (entity, tank_bubble, position, rigidbody) in query.iter() {
        let bubble_tank_id = tank_bubble.tank_id;
        let mass_position = position.value * rigidbody.mass;
        //bubble_tank_to_mass_position.insert(bubble_tank_id, mass_position);
        //bubble_tank_to_body_count.insert(bubble_tank_id, 1);

        // add or insert
        let total_mass_position = bubble_tank_to_mass_position.entry(bubble_tank_id).or_insert(FixedPointV2::zero());
        *total_mass_position += mass_position;

        let body_count = bubble_tank_to_body_count.entry(bubble_tank_id).or_insert(0);
        *body_count += 1;
    }

    for (bubble_tank_id, mass_position) in bubble_tank_to_mass_position.iter() {
        let body_count = *bubble_tank_to_body_count.get(bubble_tank_id).unwrap();
        let center_of_mass = *mass_position / FixedPoint::from(body_count);
        bubble_tank_to_center_of_mass.insert(*bubble_tank_id, center_of_mass);
    }

    // torque steer control like this:
    /*
    world.ForEachEntityWithComponent<TankBubbleComponent>((entity, tankBubbleComponent) =>
            {
                var input = new BubbleTankInput();
                var controllingPlayerId = world.GetComponent<BubbleTankComponent>(tankBubbleComponent.TankEntity).PlayerId;
                if (playerIdToInput.TryGetValue(controllingPlayerId, out var foundInput))
                {
                    input = foundInput;
                }

                var steerInput = input.SteerInput;

                var position = world.GetComponent<PositionComponent>(entity).Value;
                var com = m_TankComs[tankBubbleComponent.TankEntity];

                var delta = position - com;
                var distance = delta.GetLength();
                var direction = delta / distance;
                var directionPerpendicular = new FixedPointVector2(-direction.Y, direction.X);
                // apply torque

                var rotateForceLen = -steerInput * 12;
                var rotateForce = directionPerpendicular * rotateForceLen;

                // draw ray to debug rotation forces
                Debug.DrawRay(position - Vector3.forward * 0.65f, rotateForce, Color.red);

                // com to me
                Debug.DrawRay(com - Vector3.forward * 0.65f, position - com, Color.magenta);



                var force = rotateForce * deltaTime * (FixedPoint)(0.55f);
                if (m_TotalForceAppliedToTanks.ContainsKey(tankBubbleComponent.TankEntity) == false)
                {
                    m_TotalForceAppliedToTanks[tankBubbleComponent.TankEntity] = force;
                }
                m_TotalForceAppliedToTanks[tankBubbleComponent.TankEntity] += force;


                var velocity = world.GetComponent<VelocityComponent>(entity).Value;
                velocity += force;
                var dampenedVelocity = velocity * (FixedPoint)(0.98f);
                world.SetComponent(entity, new VelocityComponent() { Value = dampenedVelocity });
            });

            // counter forces to eliminate swaying
            world.ForEachEntityWithComponent<TankBubbleComponent>((entity, tankBubbleComponent) =>
            {
                var totalForceAppliedToTank = m_TotalForceAppliedToTanks[tankBubbleComponent.TankEntity];
                var counterForcePerBubble = totalForceAppliedToTank / (FixedPoint)m_TankBubbleCounts[tankBubbleComponent.TankEntity];

                var velocity = world.GetComponent<VelocityComponent>(entity).Value;
                velocity -= counterForcePerBubble;
                world.SetComponent(entity, new VelocityComponent() { Value = velocity });
            });
    */


    let mut tank_to_total_force_applied = HashMap::new();

    for (entity, tank_bubble, position, rigidbody) in query.iter() {
        let bubble_tank_id = tank_bubble.tank_id;
        let center_of_mass = bubble_tank_to_center_of_mass.get(&bubble_tank_id).unwrap();
        let delta = position.value - *center_of_mass;
        let distance = delta.magnitude();
        let direction = delta / distance;
        let direction_perpendicular = FixedPointV2::new(-direction.y, direction.x);
        // apply torque

        let rotate_force_len = -player_input_map.map.get(&bubble_tank_id).unwrap().steer * FixedPoint::new(12.0);
        let rotate_force = direction_perpendicular * rotate_force_len;

        let force = rotate_force * rigidbody.mass * (FixedPoint::new(0.55));
        if let Some(total_force_applied_to_tank) = bubble_tank_to_mass_position.get_mut(&bubble_tank_id) {
            *total_force_applied_to_tank += force;
        } else {
            bubble_tank_to_mass_position.insert(bubble_tank_id, force);
        }

        let velocity = rigidbody.velocity;
        let dampened_velocity = velocity * (FixedPoint::new(0.98));

    }


    
    
}
 */