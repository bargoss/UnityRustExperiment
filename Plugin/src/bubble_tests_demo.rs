use bargame_lib::{
    game_core::verlet_physics::verlet_object::VerletObject,
    game_core::math::{FixedPoint, FixedPointV2},
    game_core::verlet_physics::verlet_beam::VerletBeam,
    game_core::verlet_physics::verlet_physics_world,
    game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld,
    game_core::common::id::Id,
};
use ggez::graphics::Color;
//use hashset
use std::collections::HashSet;
// use VirtualKeyCode
use ggez::input::keyboard::KeyCode;

use crate::draw_utils::{DrawHandlers, UserBehaviour};
use crate::fixed_point_to_glm;


struct BubbleTests {
    physics_world: verlet_physics_world::VerletPhysicsWorld,
    next_id: Id,
    last_shoot: f32,
    shoot_dir_y: f64,
}

impl UserBehaviour for BubbleTests {
    fn start(&mut self) {}

    fn update(&mut self, time: f32, delta_time: f32, drawer: &mut dyn DrawHandlers, pressed_keys: &HashSet<KeyCode>) {

        if pressed_keys.contains(&KeyCode::W) {
            self.shoot_dir_y -= 0.01;
        }
        if pressed_keys.contains(&KeyCode::S) {
            self.shoot_dir_y += 0.01;
        }


        // let mut 2 buffers
        let mut buffer_a = vec![];
        let mut buffer_b = vec![];

        if (time > self.last_shoot + 1.0) {
            shoot_new_object(&mut self.physics_world, self.next_id, FixedPointV2::from_num(1.0, self.shoot_dir_y));
            self.next_id += Id::new(1);
            if self.next_id.0 > 500 {
                self.next_id = Id::new(250);
            }
            self.last_shoot = time;
        }



        central_gravity(&mut self.physics_world, FixedPoint::new(0.5));


        self.physics_world.update(FixedPoint::new(0.02), &mut buffer_a, &mut buffer_b);

        let count = self.physics_world.get_obj_iter().count();
        //println!("count: {}", count);
        self.physics_world.get_obj_iter().for_each(|obj| {
            let pos = fixed_point_to_glm::to_glam_vec2(obj.position);
            let radius = obj.radius.to_f32();
            drawer.draw_circle(pos, radius, Color::from([1.0, 1.0, 1.0, 1.0]));
        });
        self.physics_world.get_beam_iter().for_each(|beam| {
            let start_id = beam.verlet_object_id_a;
            let end_id = beam.verlet_object_id_b;
            let start_pos_fp = self.physics_world.get_object(start_id).unwrap().position;
            let end_pos_fp = self.physics_world.get_object(end_id).unwrap().position;
            let start_pos = fixed_point_to_glm::to_glam_vec2(start_pos_fp);
            let end_pos = fixed_point_to_glm::to_glam_vec2(end_pos_fp);

            drawer.draw_line(start_pos, end_pos, 0.2, Color::from([1.0, 1.0, 1.0, 1.0]));
        });
    }
}

//fn central_gravity(physics_world : verlet_physics_world::VerletPhysicsWorld, gravity : FixedPoint){
// physics world is mut
fn central_gravity(physics_world: &mut verlet_physics_world::VerletPhysicsWorld, gravity: FixedPoint) {
    let gravity_center = FixedPointV2::from_num(0.0, 0.0);
    for obj in physics_world.get_obj_iter_mut() {
        let gravity_direction = (gravity_center - obj.position).safe_normalize();
        let acceleration = gravity_direction * gravity;
        obj.acceleration += acceleration;
    }
}

fn create_beam(physics_world: &mut verlet_physics_world::VerletPhysicsWorld, id_a: Id, id_b: Id, id_beam: Id) {
    let obj_0 = physics_world.get_object(id_a).unwrap();
    let obj_1 = physics_world.get_object(id_b).unwrap();
    let distance = (obj_0.position - obj_1.position).magnitude();
    let beam = VerletBeam {
        verlet_object_id_a: id_b,
        verlet_object_id_b: id_a,
        length: distance,
    };
    physics_world.add_or_set_beam(beam, id_beam);
}

fn shoot_new_object(physics_world: &mut verlet_physics_world::VerletPhysicsWorld, id: Id, shoot_direction: FixedPointV2) {
    //physics_world.remove_object(id);

    let id_as_f64 = id.0 as f64;
    let id_mapped = (id_as_f64 % 5.0 - 2.0);
    let radius = FixedPoint::new(0.1 + 0.25 * ((id_as_f64 % 10.0) / 10.0));

    let obj = VerletObject {
        position: FixedPointV2::from_num(-20.0, 0.01 * id_as_f64),
        position_last: FixedPointV2::from_num(-20.0, 0.01 * id_as_f64),
        //acceleration: FixedPointV2::from_num(700.0, id_mapped * 15.0),
        acceleration: shoot_direction * FixedPoint::new(700.0),
        radius: radius,
        mass: radius * radius,
        is_static: false,
    };

    //println!("shoot new object with id: {}", id.0);
    physics_world.add_or_set_object(obj, id);
}

pub fn create_bubble_tests_demo() -> Box<dyn UserBehaviour> {
    let mut physics_world = verlet_physics_world::VerletPhysicsWorld::new();

    let obj0 = VerletObject {
        position: FixedPointV2::from_num(-10.0, 0.5),
        position_last: FixedPointV2::from_num(-10.0, 0.5),
        acceleration: FixedPointV2::from_num(775.1, 0.1),
        radius: FixedPoint::new(0.5),
        mass: FixedPoint::new(1.0),
        is_static: false,
    };
    physics_world.add_or_set_object(obj0, Id::new(0));

    let mut last_id = 10;
    // create obj in 5x5, centered at 0,0, seperated by 1.5
    let seperation = 1.2;
    for x in -2..3 {
        for y in -2..3 {
            let obj = VerletObject {
                position: FixedPointV2::from_num(x as f64 * seperation, y as f64 * seperation),
                position_last: FixedPointV2::from_num(x as f64 * seperation, y as f64 * seperation),
                acceleration: FixedPointV2::from_num(0.0, 0.0),
                radius: FixedPoint::new(0.599),
                mass: FixedPoint::new(1.5),
                is_static: false,
            };
            physics_world.add_or_set_object(obj, Id::new(last_id));
            last_id += 1;
        }
    }

    create_beam(&mut physics_world, Id::new(10), Id::new(11), Id::new(last_id));
    last_id += 1;
    create_beam(&mut physics_world, Id::new(11), Id::new(12), Id::new(last_id));
    last_id += 1;
    create_beam(&mut physics_world, Id::new(12), Id::new(13), Id::new(last_id));
    last_id += 1;
    create_beam(&mut physics_world, Id::new(13), Id::new(14), Id::new(last_id));
    last_id += 1;


    //BubbleTests{
    //    physics_world : physics_world,
    //    next_id: Id::new(100),
    //    last_shoot: 0.0,
    //}

    //return it
    Box::new(BubbleTests {
        physics_world: physics_world,
        next_id: Id::new(100),
        last_shoot: 0.0,
        shoot_dir_y: 0.0,
    })
}
