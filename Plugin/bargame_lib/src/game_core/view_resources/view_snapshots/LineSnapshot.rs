use std::ops::{Add, Mul};
use crate::game_core::common::Vector3;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

#[derive(Copy, Clone, Debug, Default)]
pub struct LineSnapshot {
    pub start: Vector3,
    pub end: Vector3,
    pub width: f32,
}
impl Add for LineSnapshot{
    type Output = LineSnapshot;

    fn add(self, other: LineSnapshot) -> LineSnapshot {
        LineSnapshot{
            start: self.start + other.start,
            end: self.end + other.end,
            width: self.width,
        }
    }
}
impl Mul<f32> for LineSnapshot{
    type Output = LineSnapshot;

    fn mul(self, other: f32) -> LineSnapshot {
        LineSnapshot{
            start: self.start * other,
            end: self.end * other,
            width: self.width,
        }
    }
}
impl ViewSnapshot for LineSnapshot{}