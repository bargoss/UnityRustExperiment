use std::collections::{HashMap, HashSet};
use bargame_lib::arena_fight_game::arena_game::{ArenaFightGame, ArenaFightInput};
use bargame_lib::game_core::math::{FixedPoint, FixedPointV2};
use bargame_lib::game_core::view_components::Id;
use bargame_lib::game_core::view_resources::view_snapshots::SphereSnapshot::SphereSnapshot;
use ggez::graphics::Color;
use ggez::input::keyboard::KeyCode;
use ggez::winit::event::VirtualKeyCode;
use crate::draw_utils::{DrawHandlers, UserBehaviour};

struct ArenaDemo {
    game: ArenaFightGame,
}

impl UserBehaviour for ArenaDemo {
    fn start(&mut self) {}

    fn update(&mut self, time: f32, delta_time: f32, drawer: &mut dyn DrawHandlers, pressed_keys: &HashSet<KeyCode>) {
        let mut my_input_movement_direction = FixedPointV2::from_num(0.0, 0.0);

        if pressed_keys.contains(&VirtualKeyCode::W) { my_input_movement_direction.set_y(FixedPoint::one()) }
        if pressed_keys.contains(&VirtualKeyCode::S) { my_input_movement_direction.set_y(-FixedPoint::one()) }
        if pressed_keys.contains(&VirtualKeyCode::A) { my_input_movement_direction.set_x(-FixedPoint::one()) }
        if pressed_keys.contains(&VirtualKeyCode::D) { my_input_movement_direction.set_x(FixedPoint::one()) }

        let mut input_map = HashMap::new();
        let my_input = ArenaFightInput {
            movement_direction: my_input_movement_direction,
        };
        input_map.insert(Id::new(0), my_input);

        self.game.advance_tick(input_map);
        self.game.register_keyframes();

        //let mut buffer = vec![];
        // define the vec with SphereSnapshot
        let mut buffer: Vec<SphereSnapshot> = vec![];
        self.game.sample_view_snapshots(time as f64, &mut buffer);

        for snapshot in buffer {
            drawer.draw_circle(snapshot.position, snapshot.radius, Color::RED);
        }
    }
}


pub fn create_arena_demo() -> Box<dyn UserBehaviour>{
    let game = ArenaFightGame::new();
    Box::new(ArenaDemo{game})
}