use std::collections::HashSet;
use bargame_lib::arena_fight_game::arena_game::ArenaFightGame;
use ggez::input::keyboard::KeyCode;
use ggez::winit::event::VirtualKeyCode;
use crate::draw_utils::{DrawHandlers, UserBehaviour};

struct ArenaDemo {
    game: ArenaFightGame,
}

impl UserBehaviour for ArenaDemo {
    fn start(&mut self) {}

    fn update(&mut self, time: f32, delta_time: f32, drawer: &mut dyn DrawHandlers, pressed_keys: &HashSet<KeyCode>) {
        //self.game.advance_tick()
        //self.game.draw(drawer);
    }
}


//pub fn create_arena_demo() -> Box<dyn UserBehaviour>{
//    let game = ArenaFightGame::new();
//}