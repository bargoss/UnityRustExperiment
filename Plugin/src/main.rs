mod draw_utils;

use bargame_lib::add;
use ggez::GameResult;
use ggez::glam::Vec2;
use ggez::graphics::Color;
use crate::draw_utils::{DrawHandlers, run_drawer, UserBehaviour};


struct BaransBehavior;

impl UserBehaviour for BaransBehavior {
    fn start(&mut self) {
    }

    fn update(&mut self, time: f32, delta_time: f32, drawer: &mut dyn DrawHandlers) {
        drawer.draw_circle(Vec2::new(100.0 + time, 100.0), 50.0, Color::from([1.0, 0.0, 0.0, 1.0]));
    }
}


pub fn main() -> GameResult {
    run_drawer(Some(Box::new(BaransBehavior)))
}