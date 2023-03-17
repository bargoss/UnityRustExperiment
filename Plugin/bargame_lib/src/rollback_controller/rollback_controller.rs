use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_core::components::FixedPoint;

pub trait Input where Self: Serialize + Deserialize<'static> + Default + Copy + Clone, {}
pub trait RollbackData where Self: Serialize + Deserialize<'static> + Default, {}


//pub trait RollbackControllerHandle<TInput, TRollbackData> but implements the "Default" trait as well
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

pub struct PlayerInputBuffer<TInput>
    where
        TInput: Input
{
    inputs: HashMap<u32, TInput>,
}

impl<TInput> PlayerInputBuffer<TInput>
    where
        TInput: Input
{
    pub fn new() -> Self {
        PlayerInputBuffer { inputs: HashMap::new() }
    }

    pub fn add_input(&mut self, tick: u32, input: TInput) {
        self.inputs.insert(tick, input);
    }

    pub fn clear(&mut self) {
        self.inputs.clear();
    }

    pub fn get_input(&self, tick: u32) -> Option<&TInput> {
        self.inputs.get(&tick)
    }
}

pub struct PlayersInputBuffer<TInput>
    where
        TInput: Input
{
    inputs: HashMap<u32, PlayerInputBuffer<TInput>>,
}

impl<TInput> PlayersInputBuffer<TInput>
    where
        TInput: Input
{
    pub fn new() -> Self {
        PlayersInputBuffer { inputs: HashMap::new() }
    }

    pub fn add_input(&mut self, player_id: u32, tick: u32, input: TInput) {
        let player_input_buffer = self.inputs.entry(player_id).or_insert(PlayerInputBuffer::new());
        player_input_buffer.add_input(tick, input);
    }

    pub fn clear(&mut self) {
        self.inputs.clear();
    }

    pub fn get_input(&self, player_id: u32, tick: u32) -> Option<&TInput> {
        let player_input_buffer = self.inputs.get(&player_id);
        match player_input_buffer {
            Some(player_input_buffer) => player_input_buffer.get_input(tick),
            None => None,
        }
    }

    pub fn get_all_inputs_for_tick(&self, tick: u32) -> HashMap<u32, TInput> {
        let mut inputs_for_tick = HashMap::new();
        for (player_id, player_input_buffer) in self.inputs.iter() {
            if let Some(input) = player_input_buffer.get_input(tick) {
                inputs_for_tick.insert(*player_id, *input);
            }
        }
        inputs_for_tick
    }
}


pub struct RollbackController<TInput, TRollbackData>
    where
        TInput: Input,
        TRollbackData: RollbackData
{
    rollback_data: TRollbackData,
    input_buffer: PlayersInputBuffer<TInput>,
    rollback_controller_handle: Box<dyn RollbackControllerHandle<TInput, TRollbackData>>,
}

impl<TInput, TRollbackData> RollbackController<TInput, TRollbackData>
    where
        TInput: Input,
        TRollbackData: RollbackData
{
    pub fn new(rollback_controller_handle: Box<dyn RollbackControllerHandle<TInput, TRollbackData>>) -> Self {
        RollbackController {
            rollback_data: TRollbackData::default(),
            input_buffer: PlayersInputBuffer::new(),
            rollback_controller_handle,
        }
    }

    pub fn step_game_state(&mut self) {
        let inputs_for_current_tick = self.input_buffer.get_all_inputs_for_tick(self.get_current_tick());
        self.rollback_controller_handle.step_game_state(inputs_for_current_tick);
    }

    pub fn step_game_state_predictive(&mut self) {
        let inputs_for_current_tick = self.input_buffer.get_all_inputs_for_tick(self.get_current_tick());
        self.rollback_controller_handle.step_game_state_predictive(inputs_for_current_tick);
    }

    pub fn save_rollback_data(&mut self) {
        self.rollback_data = self.rollback_controller_handle.save_rollback_data();
    }

    pub fn load_rollback_data(&mut self) {
        self.rollback_controller_handle.load_rollback_data(&self.rollback_data);
    }

    pub fn register_keyframes(&mut self) {
        self.rollback_controller_handle.register_keyframes();
    }

    pub fn update_interpolations(&mut self, viewing_time : f32) {
        self.rollback_controller_handle.update_interpolations(viewing_time);
    }

    pub fn get_current_tick(&self) -> u32 {
        self.rollback_controller_handle.get_current_tick()
    }

    pub fn get_fixed_delta_time(&self) -> FixedPoint {
        self.rollback_controller_handle.get_fixed_delta_time()
    }

    pub fn add_input(&mut self, input: TInput) {
        self.input_buffer.add_input(0, self.get_current_tick(), input);
    }
}