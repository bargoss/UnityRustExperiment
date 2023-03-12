mod draw_utils;
mod fixed_point_to_glm;

use bargame_lib::game_core::math::{FixedPoint, FixedPointV2};
use bargame_lib::game_core::verlet_physics::verlet_object::VerletObject;
use draw_utils::*;

use bargame_lib::game_core::verlet_physics::verlet_physics_world;
use bargame_lib::game_core::verlet_physics::verlet_physics_world::Id;


struct BubbleTests{
    physics_world: verlet_physics_world::VerletPhysicsWorld
}

impl UserBehaviour for BubbleTests {
    fn start(&mut self) {
    }

    fn update(&mut self, time: f32, delta_time: f32, drawer: &mut dyn DrawHandlers) {
        self.physics_world.get_obj_iter().for_each(|obj|{
            let pos = fixed_point_to_glm::to_glam_vec2(obj.position);
            let radius = obj.radius.to_f32();
            drawer.draw_circle(pos, radius, Color::from([1.0, 0.0, 0.0, 1.0]));
        });

        drawer.draw_circle(Vec2::new(100.0 + time, 100.0), 50.0, Color::from([1.0, 0.0, 0.0, 1.0]));
    }
}


pub fn main() -> GameResult {
    let mut physics_world = verlet_physics_world::VerletPhysicsWorld::new();

    let obj0 = VerletObject{
        position: FixedPointV2::new(0.0, 0.0),
        position_last: FixedPointV2::new(0.0, 0.0),
        acceleration: FixedPointV2::new(0.0, 0.0),
        radius: FixedPoint::new(0.5),
        mass: FixedPoint::new(  1.0),
        is_static: false,
    };
    physics_world.add_or_set_object(obj0, Id::new(0));

    run_drawer(Some(Box::new(BubbleTests{
        physics_world : physics_world
    })))
}