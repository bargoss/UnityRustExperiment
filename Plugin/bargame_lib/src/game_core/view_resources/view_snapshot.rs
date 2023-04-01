use std::ops::{Add, Mul};
use std::fmt::Debug;
use crate::game_core::math::FP;

pub trait ViewSnapshot where Self: Default + Debug + Send + Sync + Copy + Clone + Add<Output = Self> + Mul<FP, Output = Self>{}

pub fn interpolate_snapshots<T>(a: T, b: T, t: FP) -> T where T: ViewSnapshot {
    let a_minus = a * (-FP::one());
    a + (b + a_minus) * t
}