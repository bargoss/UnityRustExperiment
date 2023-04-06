pub mod game_core;
//pub mod rollback_controller;
pub mod bubble_tanks_game;
pub mod arena_fight_game;

pub mod example {
    //include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
