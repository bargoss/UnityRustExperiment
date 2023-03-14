use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyModel {
    pub my_int: i32,
}

/*
pub struct RollbackController {
    predicted_entities_rollback_state: GameState,
    previous_state: GameState,
}
 */

// a trait that requires serde::Serialize and serde::Deserialize
//pub trait RollbackState: Serialize + Deserialize {}

/*
pub trait RollbackControllerHandle{
    fn update(&mut self, delta_time: f32);
    fn serialize_rollback_entities(&mut self,
    fn get_current_state(&self) -> &GameState;
    fn get_previous_state(&self) -> &GameState;
    fn get_current_state_mut(&mut self) -> &mut GameState;
    fn get_previous_state_mut(&mut self) -> &mut GameState;
}

 */