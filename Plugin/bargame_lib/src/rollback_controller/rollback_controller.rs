use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_core::components::FixedPoint;

pub trait Input where Self: Serialize + Deserialize<'static> + Default + Copy + Clone, {}
pub trait RollbackData where Self: Serialize + Deserialize<'static> + Default, {}

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

pub struct InputPackage<TInput>
    where
        TInput: Input
{
    pub tick: u32,
    pub player_id: u32,
    pub input: TInput,
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

    fn get_target_tick(&self, network_time: f64) -> u32 {
        let div = (network_time / self.rollback_controller_handle.get_fixed_delta_time().to_f64());
        let target_tick = div as u32;
        target_tick
    }

    pub fn on_update(&mut self, network_time: f64, local_player_input: TInput){
        let target_tick = self.get_target_tick(network_time);
        
    }

    fn simulate_until(&mut self, tick: u32) {
        let mut current_tick = self.rollback_controller_handle.get_current_tick();
        while current_tick < tick {
            let inputs = self.input_buffer.get_all_inputs_for_tick(current_tick);
            self.rollback_controller_handle.step_game_state(inputs);
            current_tick = self.rollback_controller_handle.get_current_tick();
        }
    }

    pub fn register_inputs(&mut self, inputs: Vec<InputPackage<TInput>>) {
        for input in inputs {
            self.input_buffer.add_input(input.player_id, input.tick, input.input);
        }
    }
}
