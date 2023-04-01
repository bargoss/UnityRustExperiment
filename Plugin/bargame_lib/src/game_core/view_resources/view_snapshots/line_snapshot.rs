use std::ops::{Add, Mul};
use crate::game_core::math::{FP, FP3};
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

#[derive(Copy, Clone, Debug, Default)]
pub struct LineSnapshot {
    pub start: FP3,
    pub end: FP3,
    pub width: FP,
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
impl Mul<FP> for LineSnapshot{
    type Output = LineSnapshot;

    fn mul(self, other: FP) -> LineSnapshot {
        LineSnapshot{
            start: self.start * other,
            end: self.end * other,
            width: self.width,
        }
    }
}
impl ViewSnapshot for LineSnapshot{}