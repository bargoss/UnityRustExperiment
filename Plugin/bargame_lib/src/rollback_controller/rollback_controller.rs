use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_core::components::FixedPoint;

pub trait Input where Self: Serialize + Deserialize<'static>, {}
pub trait RollbackData where Self: Serialize + Deserialize<'static>, {}


pub trait RollbackControllerHandle<TInput, TRollbackData>
    where
        TInput: Input,
        TRollbackData: RollbackData
{
    fn save_rollback_data(&mut self) -> TRollbackData;
    fn load_rollback_data(&mut self, game_state: TRollbackData);

    fn step_game_state(&mut self, inputs: HashMap<u32,TInput>);
    fn step_game_state_predictive(&mut self, inputs: HashMap<u32,TInput>);

    fn get_current_tick(&self) -> u32;
    fn get_fixed_delta_time(&self) -> FixedPoint;

    fn register_keyframes(&mut self);
    fn update_interpolations(&mut self, viewing_time : f32);
}

pub struct InputBuffer<TInput>
    where
        TInput: Input
{
    inputs: HashMap<u32, TInput>,
}

pub struct RollbackController<TInput, TRollbackData>
    where
        TInput: Input,
        TRollbackData: RollbackData
{
    rollback_data: TRollbackData,
    input_buffer: InputBuffer<TInput>,
}