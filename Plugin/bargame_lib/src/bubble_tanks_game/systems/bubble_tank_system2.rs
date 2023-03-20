/*
using System.Collections.Generic;
using BubbleTanks.Components;
using Core;
using Core.Components;
using Core.FPMath;
using DefaultNamespace.BubbleTanks;
using UnityEngine;

namespace BubbleTanks.Systems
{
    public static class BubbleTankSystem
    {
        private static Dictionary<int, FixedPointVector2> m_TankBubblePosSums = new Dictionary<int, FixedPointVector2>();
        private static Dictionary<int, int> m_TankBubbleCounts = new Dictionary<int, int>();
        private static Dictionary<int, FixedPointVector2> m_TankComs = new Dictionary<int, FixedPointVector2>();

        public static void Run(World world, FixedPoint deltaTime, Dictionary<int, BubbleTankInput> playerIdToInput)
        {
            UpdateTankComs(world);

            HandleSteerTorque(world, deltaTime, playerIdToInput);
        }

        private static Dictionary<int, FixedPointVector2> m_TotalForceAppliedToTanks = new Dictionary<int, FixedPointVector2>();
        private static void HandleSteerTorque(World world, FixedPoint deltaTime, Dictionary<int, BubbleTankInput> playerIdToInput)
        {
            m_TotalForceAppliedToTanks.Clear();
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
        }

        private static void UpdateTankComs(World world)
        {
            m_TankBubblePosSums.Clear();
            m_TankBubbleCounts.Clear();
            m_TankComs.Clear();

            world.ForEachEntityWithComponent<TankBubbleComponent>((entity, tankBubbleComponent) =>
            {
                var position = world.GetComponent<PositionComponent>(entity).Value;
                var tankEntity = tankBubbleComponent.TankEntity;

                if (m_TankBubblePosSums.ContainsKey(tankEntity) == false)
                {
                    m_TankBubblePosSums[tankEntity] = FixedPointVector2.Zero;
                }
                if (m_TankBubbleCounts.ContainsKey(tankEntity) == false)
                {
                    m_TankBubbleCounts[tankEntity] = 0;
                }

                m_TankBubblePosSums[tankEntity] += position;
                m_TankBubbleCounts[tankEntity]++;
            });

            foreach (var tankEntity in m_TankBubblePosSums.Keys)
            {
                m_TankComs[tankEntity] = m_TankBubblePosSums[tankEntity] / (FixedPoint)m_TankBubbleCounts[tankEntity];
            }
        }
    }
}
*/

// convert to rust and bevy_ecs

use std::collections::HashMap;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Query, Res};
use crate::bubble_tanks_game::BubbleTanksInput;
use crate::bubble_tanks_game::components::bubble_tank::BubbleTank;
use crate::bubble_tanks_game::components::tank_bubble::TankBubble;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::game_world::PlayerInputMap;
use crate::game_core::math::FixedPointV2;
use crate::game_core::resources::id_entity_map::IdEntityMap;

//pub fn bubble_tank_system(
// tank_bubble_query: Query<(&TankBubble, &Position, &mut Rigidbody)>,
// bubble_tank_query: Query<(&BubbleTank, &)>,
// player_input_map: Res<PlayerInputMap<BubbleTanksInput>>,
// id_entity_map: Res<IdEntityMap>,
//)
/*
pub fn bubble_tank_system(
    tank_bubble_query: Query<(&TankBubble, &Position, &mut Rigidbody)>,
    bubble_tank_query: Query<(Entity, &BubbleTank)>,
    player_input_map: Res<PlayerInputMap<BubbleTanksInput>>,
    id_entity_map: Res<IdEntityMap>,
) {
    // update tank coms
    let mut tank_bubble_pos_sums = HashMap::new();
    let mut tank_bubble_counts = HashMap::new();
    let mut tank_coms = HashMap::new();

    for (tank_bubble_component, position_component) in tank_bubble_query.iter() {
        let tank_entity = tank_bubble_component.tank_id;
        if tank_bubble_pos_sums.contains_key(&tank_entity) == false {
            tank_bubble_pos_sums.insert(tank_entity, FixedPointV2::zero());
        }
        if tank_bubble_counts.contains_key(&tank_entity) == false {
            tank_bubble_counts.insert(tank_entity, 0);
        }

        let tank_bubble_pos_sum = tank_bubble_pos_sums.get_mut(&tank_entity).unwrap();
        *tank_bubble_pos_sum += position_component.value;
        let tank_bubble_count = tank_bubble_counts.get_mut(&tank_entity).unwrap();
        *tank_bubble_count += 1;
    }

    for tank_entity in tank_bubble_pos_sums.keys() {
        let tank_bubble_pos_sum = tank_bubble_pos_sums.get(tank_entity).unwrap();
        let tank_bubble_count = tank_bubble_counts.get(tank_entity).unwrap();
        let tank_com = tank_bubble_pos_sum / *tank_bubble_count;
        tank_coms.insert(*tank_entity, tank_com);
    }

    // handle steer torque
    let mut total_force_applied_to_tanks = HashMap::new();
    //(&TankBubble, &Position, &mut Rigidbody)
    for (&tank_bubble, &position, &rigidbody) in tank_bubble_query.iter() {
        let input = BubbleTanksInput::default();
        let tank_id = tank_bubble.tank_id;
        let q_result = match id_entity_map.get_query_result(tank_id, bubble_tank_query) {
            Ok(q_result) => q_result,
            Err(_) => continue,
        };
        let asdas = tank_bubble_query.get()
        let controlling_player_id = world
            .get_component::<BubbleTank>(tank_bubble.tank_entity)
            .unwrap()
            .player_id;
        if let Some(found_input) = player_id_to_input.get(&controlling_player_id) {
            input = *found_input;
        }

        let steer_input = input.steer_input;

        let position = position.value;
        let com = tank_coms.get(&tank_bubble.tank_entity).unwrap();

        let delta = position - com;
        let distance = delta.get_length();
        let direction = delta / distance;
        let direction_perpendicular = FixedPointV2::new(-direction.y, direction.x);
        // apply torque

        let rotate_force_len = -steer_input * 12;
        let rotate_force = direction_perpendicular * rotate_force_len;

        // draw ray to debug rotation forces
        // Debug.DrawRay(position - Vector3.forward * 0.65f, rotate_force, Color.red);

        // com to me
        // Debug.DrawRay(com - Vector3.forward * 0.65f, position - com, Color.magenta);

        let force = rotate_force * delta_time * 0.55;
        if total_force_applied_to_tanks.contains_key(&tank_bubble.tank_entity) == false
        {
            total_force_applied_to_tanks.insert(
                tank_bubble.tank_entity,
                force,
            );
        }
        let total_force_applied_to_tank =
            total_force_applied_to_tanks.get_mut(&tank_bubble.tank_entity).unwrap();
        *total_force_applied_to_tank += force;

        let velocity = rigidbody_component.velocity;
        velocity += force;
        let dampened_velocity = velocity * 0.98;
        rigidbody_component.velocity = dampened_velocity;
    }

    // handle counter force
    for (tank_bubble_component, position_component) in tank_bubble_query.iter() {
        let total_force_applied_to_tank =
            total_force_applied_to_tanks.get(&tank_bubble_component.tank_entity).unwrap();
        let counter_force_per_bubble = total_force_applied_to_tank / tank_bubble_counts[&tank_bubble_component.tank_entity];

        let velocity = rigidbody_component.velocity;
        velocity -= counter_force_per_bubble;
        rigidbody_component.velocity = velocity;
    }
}
*/
