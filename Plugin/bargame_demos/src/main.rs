use ggez::GameResult;
use crate::draw_utils::run_game;
mod draw_utils;
mod fixed_point_to_glm;
mod bubble_tests_demo;
use crate::bubble_tests_demo::create_bubble_tests_demo;

mod arena_demo;
use crate::arena_demo::create_arena_demo;

//pub fn main() -> GameResult { run_game(Some(create_bubble_tests_demo())) }
pub fn main() -> GameResult { run_game(Some(create_arena_demo())) }