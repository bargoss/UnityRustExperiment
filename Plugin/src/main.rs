mod draw_utils;
mod fixed_point_to_glm;

use bargame_lib::game_core::math::{FixedPoint, FixedPointV2};
use bargame_lib::game_core::verlet_physics::verlet_object::VerletObject;
use draw_utils::*;

use bargame_lib::game_core::verlet_physics::verlet_physics_world;
use bargame_lib::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;
use bargame_lib::game_core::verlet_physics::verlet_physics_world::Id;


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
    }
}


pub fn main() -> GameResult {
    let mut physics_world = verlet_physics_world::VerletPhysicsWorld::new();

    let obj0 = VerletObject{
        position: FixedPointV2::new(-10.0, 0.5),
        position_last: FixedPointV2::new(-10.0, 0.5),
        acceleration: FixedPointV2::new(475.1, 0.1),
        radius: FixedPoint::new(0.5),
        mass: FixedPoint::new(  1.0),
        is_static: false,
    };
    physics_world.add_or_set_object(obj0, Id::new(0));

    let mut last_id = 10;
    // create obj in 5x5, centered at 0,0, seperated by 1.5
    let seperation = 1.0;
    for x in -2..3 {
        for y in -2..3 {
            let obj = VerletObject{
                position: FixedPointV2::new(x as f64 * seperation, y as f64 * seperation),
                position_last: FixedPointV2::new(x as f64 * seperation, y as f64 * seperation),
                acceleration: FixedPointV2::new(0.0, 0.0),
                radius: FixedPoint::new(0.5),
                mass: FixedPoint::new(  1.0),
                is_static: false,
            };
            physics_world.add_or_set_object(obj, Id::new(last_id));

            last_id+=1;
        }
    }


    run_drawer(Some(Box::new(BubbleTests{
        physics_world : physics_world
    })))
}