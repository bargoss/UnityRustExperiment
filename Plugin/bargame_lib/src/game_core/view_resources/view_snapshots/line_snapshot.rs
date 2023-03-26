use std::ops::{Add, Mul};
use crate::game_core::math::{FixedPoint, FixedPointV3};
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

#[derive(Copy, Clone, Debug, Default)]
pub struct LineSnapshot {
    pub start: FixedPointV3,
    pub end: FixedPointV3,
    pub width: FixedPoint,
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
impl Mul<FixedPoint> for LineSnapshot{
    type Output = LineSnapshot;

    fn mul(self, other: FixedPoint) -> LineSnapshot {
        LineSnapshot{
            start: self.start * other,
            end: self.end * other,
            width: self.width,
        }
    }
}
impl ViewSnapshot for LineSnapshot{}