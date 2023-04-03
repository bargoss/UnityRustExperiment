use nalgebra::{Point2, RealField, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
//use nphysics_testbed2d::Testbed;
//use nphysics2d::world::DefaultMechanicalWorld;



#[cfg(test)]
mod tests{
    //use nphysics2d::nalgebra::Vector2;
    //use simba::scalar::FixedI32F32;
    use super::*;



    #[test]
    fn test_physics(){
        // FixedI16F16 equivalent of 2.5:

        //let nalgebra_fixed_point_vector = nalgebra::Vector2::new(FixedI16F16::from_num(2.5), FixedI16F16::from_num(2.5));
        // nalgebra vector:
        //let nalgebra_fixed_point_vector = nalgebra::<FixedI32F32>::Vector2::new(2.5, 2.5);
        // define a ComplexField

        let zero = nalgebra::convert::<f64, simba::scalar::FixedI40F24>(0.0);
        let one = nalgebra::convert::<f64, simba::scalar::FixedI40F24>(1.0);
        let num05 = nalgebra::convert::<f64, simba::scalar::FixedI40F24>(0.5);
        let gravity_vec_nphysics = nphysics2d::nalgebra::Matrix2x1::new(zero, zero);

        //let mechanical_world = DefaultMechanicalWorld::new(gravity_vec_nphysics);

        let my_vec_0 = Vector2::new(one, num05);


        //let nalgebra_fixed_point_vector = nalgebra::Vector2::new(FixedI32F32::from_num(2.5), FixedI32F32::from_num(2.5));
        //let simba_num = simba::scalar::FixedI32F32::from_num(2.5);
        //let simba_vector = Vector2::new(FixedI32F32::from_num(2.5), FixedI32F32::from_num(2.5));
        //let simba_vector_normalized = simba_vector.normalize();
        //let simba_vector2 = Vector2::new(FixedI32F32::from_num(1.0), FixedI32F32::from_num(0.5));
        //let dot_product = simba_vector.dot(&simba_vector2);

        // print it
        //println!("simba_num: {:?}", dot_product);




        //let nphysics_fixed_point_vector = nphysics2d::math::Vector::new(FixedI32F32::from_num(0.0), FixedI32F32::from_num(-9.81));
        //let realfield = nphysics_fixed_point_vector.
        //let gravity = nphysics_fixed_point_vector;

        //let num1 = FixedI16F16::from_num(1.0);
        //let gravity = Vector2::<FixedI16F16>::new(FixedI16F16::from_num(0),FixedI16F16::from_num(-9.81));
        //let mechanical_world = DefaultMechanicalWorld::new(simba_vector);
        //let geometrical_world = DefaultGeometricalWorld::new();
        //let mut bodies = DefaultBodySet::new();
        //let mut colliders = DefaultColliderSet::new();
        //let joint_constraints = DefaultJointConstraintSet::new();
        //let force_generators = DefaultForceGeneratorSet::new();

        let sdadas = 3;
    }


    //#[test]
    //fn fixed_point_example(){
    //    let num1 = FixedI32F32::from_bits(0x0001);
    //    let a = Vector2::<FixedI32F32>::new(num1, num1);
    //    let b = a + a;
    //    let dadsa = 3;
    //    //let a = Vector2::<FixedI16F16>::new(FixedI16F16::from(1.0), FixedI16F16::from(2.0));
    //    //let b = Vector2::<FixedI16F16>::new(FixedI16F16::from(3.0), FixedI16F16::from(4.0));
    //    //let c = a + b;
    //    //println!("c = {:?}", c);
    //}

}