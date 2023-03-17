use std::collections::HashMap;
use crate::game_core::math::FixedPoint;
use crate::rollback_controller::input::Input;
use crate::rollback_controller::rollback_data::RollbackData;

pub trait RollbackControllerHandle<TInput, TRollbackData>
    where
        TInput: Input,
        TRollbackData: RollbackData
{
    fn save_rollback_data(&mut self) -> TRollbackData;
    fn load_rollback_data(&mut self, game_state: &TRollbackData);

    fn step_game_state(&mut self, inputs: HashMap<u32,TInput>);
    fn step_game_state_predictive(&mut self, inputs: HashMap<u32,TInput>);

    fn get_current_tick(&self) -> u32;
    fn get_fixed_delta_time(&self) -> FixedPoint;

    fn register_keyframes(&mut self);
    fn update_interpolations(&mut self, viewing_time : f32);
}