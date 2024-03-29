//use nalgebra::{Isometry2, Point2, Point3, RealField, Vector2};
use ncollide2d::query::Proximity;
use ncollide2d::shape::{Compound, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::math::Velocity;
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

/*
 * NOTE: The `r` macro is only here to convert from f64 to the `N` scalar type.
 * This simplifies experimentation with various scalar types (f32, fixed-point numbers, etc.)
 */
pub fn test_physics() {
    /*
     * World
     */
    //let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(r!(0.0), r!(-9.81)));
    //let gravity = nalgebra::zero();

    // create a nalgebra vector2:
    // let gravity = nalgebra::Vector2::new(0,-10);

    nalgebra::distance()
    let gravity = nalgebra::Vector2::new(0,-10);
    let mechanical_world = DefaultMechanicalWorld::new(gravity);
    let geometrical_world = DefaultGeometricalWorld::new();
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let joint_constraints = DefaultJointConstraintSet::new();
    let force_generators = DefaultForceGeneratorSet::new();

    /*
     * Ground
     *
     * NOTE: Enabling CCD on the walls here won't change anything since
     * in this demo we chose to enable CCD on every collider attached to
     * dynamic rigid bodies (to show that CCD between moving colliders work
     * as well).
     */
    //let ground_size = r!(25.0);

    // create nalgebra scalar
    let ground_size = nalgebra::one() * 25;
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, r!(0.1))));

    let ground_handle = bodies.insert(Ground::new());
    let co = ColliderDesc::new(ground_shape.clone()).build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape.clone())
        .position(Isometry2::new(
            Vector2::new(r!(-3.0), r!(0.0)),
            r!(3.14) / r!(2.0),
        ))
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape.clone())
        .position(Isometry2::new(
            Vector2::new(r!(6.0), r!(0.0)),
            r!(3.14) / r!(2.0),
        ))
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape.clone())
        .position(Isometry2::translation(r!(0.0), r!(10.0)))
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    // Add a sensor, to show that CCD works on sensors too.
    let co = ColliderDesc::new(ground_shape)
        .position(Isometry2::new(
            Vector2::new(r!(2.5), r!(0.0)),
            r!(3.14) / r!(2.0),
        ))
        .sensor(true)
        .build(BodyPartHandle(ground_handle, 0));
    let sensor_handle = colliders.insert(co);
    testbed.set_collider_color(sensor_handle, Point3::new(1.0, 1.0, 0.0));

    /*
     * Create the shapes
     */
    let num = 5;
    //    let mut rady = 0.1;
    //    let mut radx = rady * 4.0;
    let rady;
    let radx;

    //    let shape = {
    //        let mut cross_geoms = Vec::new();
    //
    //        let large_rad = 0.4f32;
    //        let small_rad = 0.05f32;
    //
    //        radx = large_rad;
    //        rady = large_rad;
    //
    //        let edge_x = Cuboid::new(Vector2::new(large_rad, small_rad));
    //        let edge_y = Cuboid::new(Vector2::new(small_rad, large_rad));
    //
    //        cross_geoms.push((na::one(), ShapeHandle::new(edge_x)));
    //        cross_geoms.push((na::one(), ShapeHandle::new(edge_y)));
    //
    //        let compound = Compound::new(cross_geoms);
    //        ShapeHandle::new(compound)
    //    };

    let shape = {
        let large_rad = r!(0.4);
        let small_rad = r!(0.05);

        radx = large_rad;
        rady = large_rad;

        let delta1 = Isometry2::new(Vector2::new(r!(0.0), large_rad - small_rad), nalgebra::zero());
        let delta2 = Isometry2::new(Vector2::new(-large_rad + small_rad, r!(0.0)), nalgebra::zero());
        let delta3 = Isometry2::new(Vector2::new(large_rad - small_rad, r!(0.0)), nalgebra::zero());

        let mut compound_geoms = Vec::new();
        let vertical = ShapeHandle::new(Cuboid::new(Vector2::new(small_rad, large_rad)));
        let horizontal = ShapeHandle::new(Cuboid::new(Vector2::new(large_rad, small_rad)));
        compound_geoms.push((delta1, horizontal));
        compound_geoms.push((delta2, vertical.clone()));
        compound_geoms.push((delta3, vertical));

        let compound = Compound::new(compound_geoms);
        ShapeHandle::new(compound)
    };

    //    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(radx, rady)));
    //    let shape = ShapeHandle::new(Ball::new(rady));

    let shiftx = (radx + ColliderDesc::<N>::default_margin() + r!(0.005)) * r!(2.0);
    let shifty = (rady + ColliderDesc::<N>::default_margin() + r!(0.005)) * r!(2.0);
    let centerx = shiftx * r!(num as f64) / r!(2.0) - r!(0.5);
    let centery = shifty / r!(2.0) + r!(4.0);

    for i in 0usize..num {
        for j in 0..num {
            let x = r!(i as f64) * shiftx - centerx;
            let y = r!(j as f64) * shifty + centery;

            // Build the rigid body.
            let rb = RigidBodyDesc::new()
                .translation(Vector2::new(x, y))
                .velocity(Velocity::linear(r!(100.0), r!(-10.0)))
                .build();
            let rb_handle = bodies.insert(rb);

            // Build the collider.
            let co = ColliderDesc::new(shape.clone())
                .ccd_enabled(true)
                .density(r!(1.0))
                .build(BodyPartHandle(rb_handle, 0));
            colliders.insert(co);

            testbed.set_body_color(rb_handle, Point3::new(0.5, 0.5, 1.0));
        }
    }

    /*
     * Set up the testbed.
     */

    // Callback that will be executed on the main loop to handle proximities by recoloring bodies.
    testbed.add_callback(move |_, geometrical_world, _, colliders, graphics, _| {
        for prox in geometrical_world.proximity_events() {
            let c1 = colliders.get(prox.collider1).unwrap();
            let c2 = colliders.get(prox.collider2).unwrap();
            let body1 = c1.body();
            let body2 = c2.body();

            let color = match prox.new_status {
                Proximity::WithinMargin | Proximity::Intersecting => Point3::new(1.0, 1.0, 0.0),
                Proximity::Disjoint => {
                    if c1.position().translation.vector.x > c2.position().translation.vector.x {
                        if body1 == ground_handle {
                            Point3::new(0.5, 0.5, 1.0)
                        } else {
                            Point3::new(0.5, 1.0, 0.5)
                        }
                    } else {
                        if body1 == ground_handle {
                            Point3::new(0.5, 1.0, 0.5)
                        } else {
                            Point3::new(0.5, 0.5, 1.0)
                        }
                    }
                }
            };

            if body1 != ground_handle {
                graphics.set_body_color(body1, color);
            }

            if body2 != ground_handle {
                graphics.set_body_color(body2, color);
            }
        }
    });

    testbed.set_ground_handle(Some(ground_handle));
    testbed.set_world(
        mechanical_world,
        geometrical_world,
        bodies,
        colliders,
        joint_constraints,
        force_generators,
    );
    testbed.look_at(Point2::new(3.0, 5.0), 95.0);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        let testbed = Testbed::<f32>::from_builders(0, vec![("CCD", init_world)]);
        testbed.run()
    }

    //#[test]
    //fn test_fixed_point_physics() {
    //    // create a test example
    //    let mut world = World::new();
    //}
}


