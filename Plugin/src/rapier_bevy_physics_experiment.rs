use rapier2d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier2d::na::{Vector2};
use rapier2d::prelude::{IntegrationParameters, PhysicsPipeline, RigidBodyHandle, RigidBodyPosition};



#[cfg(test)]
mod tests {
    use rapier2d::dynamics::{CCDSolver, ImpulseJointSet, MultibodyJointSet};
    use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};
    use rapier2d::na::vector;
    use rapier2d::prelude::{IslandManager, QueryPipeline};
    use rapier2d::prelude::RigidBodyType::Dynamic;
    use super::*;


    #[test]
    fn rapier_test_2(){
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vector2::new(0.0, 10.0))
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        /* Create other structures necessary for the simulation. */
        //let gravity = vector![0.0, -9.81];
        let gravity = Vector2::new(0.0, -9.81);
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        /* Run the game loop, stepping the simulation once per frame. */
        for _ in 0..200 {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                None,
                &physics_hooks,
                &event_handler,
            );

            let ball_body = &rigid_body_set[ball_body_handle];
            println!(
                "Ball altitude: {}",
                ball_body.translation().y
            );
        }
    }

    #[test]
    fn rapier_test() {
        // Set up the physics engine
        let gravity = Vector2::new(0.0, -9.81);
        let mut integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::default();
        let mut rigid_body_set = RigidBodySet::default();
        let mut island_manager = IslandManager::default();
        let mut broad_phase = Default::default();
        let mut narrow_phase = Default::default();
        let mut bodies = Default::default();
        let mut colliders = Default::default();
        let mut impulse_joints = Default::default();
        let mut multibody_joints = Default::default();
        let mut ccd_solver = Default::default();
        let mut query_pipeline = QueryPipeline::default();

        //let mut rigid_body_handle = rigid_body_set.insert(RigidBodyBuilder::new(Dynamic).additional_mass(5.0).build());
        let mut rigid_body_handle = rigid_body_set.insert(rapier2d::prelude::RigidBodyBuilder::dynamic().additional_mass(0.1).build());

        //// Create a rigid body with a position and a velocity
        //let rigid_body_handle: RigidBodyHandle = rigid_body_set.insert(
        //    RigidBodyBuilder::new_dynamic()
        //        .translation(Vector2::new(0.0, 0.0))
        //        .linvel(Vector2::new(0.0, 0.0))
        //        .build()
        //);

        // Apply a force to the rigid body
        let force = Vector2::new(1000.0, 1000.0);
        rigid_body_set
            .get_mut(rigid_body_handle)
            .unwrap()
            .add_force(force, true);


        let position_before = rigid_body_set.get(rigid_body_handle).unwrap().position().clone();

        let initial_isometry = rapier2d::na::Isometry2::new(Vector2::new(-1.0, -1.0), rapier2d::na::zero());
        rigid_body_set
            .get_mut(rigid_body_handle)
            .unwrap()
            .set_position(initial_isometry, true);


        let dt = 0.1;
        for i in 0..10 {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut bodies,
                &mut colliders,
                &mut impulse_joints,
                &mut multibody_joints,
                &mut ccd_solver,
                None,
                &mut (),
                &mut (),
            );
        }

        let position_after = rigid_body_set.get(rigid_body_handle).unwrap().position().clone();

        println!("Position before: {:?}", position_before);
        println!("Position after: {:?}", position_after);
    }
}
