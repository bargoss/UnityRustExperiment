use std::ops::{Add, Mul};
use std::fmt::Debug;
use crate::game_core::math::FixedPoint;

pub trait ViewSnapshot where Self: Default + Debug + Send + Sync + Copy + Clone + Add<Output = Self> + Mul<FixedPoint, Output = Self>{}

pub fn interpolate_snapshots<T>(a: T, b: T, t: FixedPoint) -> T where T: ViewSnapshot {
    let a_minus = a * (-FixedPoint::one());
    a + (b + a_minus) * t
}