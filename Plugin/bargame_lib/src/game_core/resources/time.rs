use crate::game_core::math::FixedPoint;

pub struct Time{
    pub fixed_delta_time : FixedPoint,
    pub tick: u32
}