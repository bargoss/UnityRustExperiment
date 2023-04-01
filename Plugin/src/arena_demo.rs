use std::collections::{HashMap, HashSet};
use bargame_lib::arena_fight_game::arena_game::*;
use bargame_lib::game_core::math::*;
use bargame_lib::game_core::common::*;
use bargame_lib::game_core::components::NetId;
use bargame_lib::game_core::view_resources::view_snapshots::*;
use ggez::glam::Vec3;
use ggez::graphics::Color;
use ggez::input::keyboard::KeyCode;
use ggez::winit::event::VirtualKeyCode;
use crate::draw_utils::{DrawHandlers, UserBehaviour, Vec2};

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
        //let my_input = ArenaInput {
        //    movement_direction: my_input_movement_direction,
        //};
        //input_map.insert(Id::new(0), my_input);

        input_map.insert(Id::new(0), ArenaInput{
            node_drag_drop: Some(NodeDragDropInput {
                source_node_net_id: NetId { value: Id::new(0) },
                target_node_net_id: NetId { value: Id::new(1) },
        })});

        self.game.advance_tick(input_map);
        self.game.register_keyframes();

        let game_time = FixedPoint::new(self.game.get_tick() as f64) * self.game.get_fixed_delta_time();

        //let mut buffer = vec![];
        // define the vec with SphereSnapshot
        let mut buffer: Vec<SphereSnapshot> = vec![];
        self.game.sample_view_snapshots(game_time, &mut buffer);

        for snapshot in buffer {
            let position = Vec2::new(snapshot.position.x().to_f32(), snapshot.position.y().to_f32());
            let radius = snapshot.radius.to_f32();
            drawer.draw_circle(position, radius, Color::RED);
        }
    }
}


pub fn create_arena_demo() -> Box<dyn UserBehaviour>{
    let mut game = ArenaFightGame::default();
    //game.add_player_character(Id::new(0), FixedPointV2::from_num(1.5, 1.6));
    Box::new(ArenaDemo{game})
}