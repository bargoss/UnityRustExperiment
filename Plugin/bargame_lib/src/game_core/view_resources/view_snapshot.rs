use std::ops::{Add, Mul};

pub trait ViewSnapshot where Self: Default + Copy + Clone + Add<Output = Self> + Mul<f32, Output = Self>{}

pub fn interpolate_snapshots<T>(a: T, b: T, t: f32) -> T where T: ViewSnapshot {
    let a_minus = a * -1.0;
    a + (b + a_minus) * t
}