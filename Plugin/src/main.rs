mod draw_utils;
mod fixed_point_to_glm;

use bargame_lib::{
    game_core::verlet_physics::verlet_object::VerletObject,
    game_core::math::{FixedPoint, FixedPointV2},
    game_core::verlet_physics::verlet_beam::VerletBeam,
    game_core::verlet_physics::verlet_physics_world,
    game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld,
    game_core::verlet_physics::verlet_physics_world::Id
};
use draw_utils::*;


struct BubbleTests{
    physics_world: verlet_physics_world::VerletPhysicsWorld
}

impl UserBehaviour for BubbleTests {
    fn start(&mut self) {
    }

    fn update(&mut self, time: f32, delta_time: f32, drawer: &mut dyn DrawHandlers) {
        // let mut 2 buffers
        let mut buffer_a = vec![];
        let mut buffer_b = vec![];


        self.physics_world.update(FixedPoint::new(0.02), &mut buffer_a, &mut buffer_b);

        self.physics_world.get_obj_iter().for_each(|obj|{
            let pos = fixed_point_to_glm::to_glam_vec2(obj.position);
            let radius = obj.radius.to_f32();
            drawer.draw_circle(pos, radius, Color::from([1.0, 1.0, 1.0, 1.0]));
        });
        self.physics_world.get_beam_iter().for_each(|beam|{
            let start_id = beam.verlet_object_id_a;
            let end_id = beam.verlet_object_id_b;
            let start_pos_fp = self.physics_world.get_object(start_id).unwrap().position;
            let end_pos_fp = self.physics_world.get_object(end_id).unwrap().position;
            let start_pos = fixed_point_to_glm::to_glam_vec2(start_pos_fp);
            let end_pos = fixed_point_to_glm::to_glam_vec2(end_pos_fp);

            drawer.draw_line(start_pos, end_pos, 0.2,Color::from([1.0, 1.0, 1.0, 1.0]));
        });

    }
}

fn create_beam(physics_world: &mut verlet_physics_world::VerletPhysicsWorld, id_a: Id, id_b: Id, id_beam : Id){
    let obj_0 = physics_world.get_object(id_a).unwrap();
    let obj_1 = physics_world.get_object(id_b).unwrap();
    let distance = (obj_0.position - obj_1.position).magnitude();
    let beam = VerletBeam{
        verlet_object_id_a: id_b,
        verlet_object_id_b: id_a,
        length: distance,
    };
    physics_world.add_or_set_beam(beam, id_beam);
}

pub fn main() -> GameResult {
    let mut physics_world = verlet_physics_world::VerletPhysicsWorld::new();

    let obj0 = VerletObject{
        position: FixedPointV2::from_num(-10.0, 0.5),
        position_last: FixedPointV2::from_num(-10.0, 0.5),
        acceleration: FixedPointV2::from_num(475.1, 0.1),
        radius: FixedPoint::new(0.5),
        mass: FixedPoint::new(  1.0),
        is_static: false,
    };
    physics_world.add_or_set_object(obj0, Id::new(0));

    let mut last_id = 10;
    // create obj in 5x5, centered at 0,0, seperated by 1.5
    let seperation = 1.2;
    for x in -2..3 {
        for y in -2..3 {
            let obj = VerletObject{
                position: FixedPointV2::from_num(x as f64 * seperation, y as f64 * seperation),
                position_last: FixedPointV2::from_num(x as f64 * seperation, y as f64 * seperation),
                acceleration: FixedPointV2::from_num(0.0, 0.0),
                radius: FixedPoint::new(0.5),
                mass: FixedPoint::new(  1.0),
                is_static: false,
            };
            physics_world.add_or_set_object(obj, Id::new(last_id));
            last_id+=1;
        }
    }

    create_beam(&mut physics_world, Id::new(10), Id::new(11), Id::new(last_id));
    last_id+=1;
    create_beam(&mut physics_world, Id::new(11), Id::new(12), Id::new(last_id));
    last_id+=1;
    create_beam(&mut physics_world, Id::new(12), Id::new(13), Id::new(last_id));
    last_id+=1;
    create_beam(&mut physics_world, Id::new(13), Id::new(14), Id::new(last_id));
    last_id+=1;



    run_drawer(Some(Box::new(BubbleTests{
        physics_world : physics_world
    })))
}