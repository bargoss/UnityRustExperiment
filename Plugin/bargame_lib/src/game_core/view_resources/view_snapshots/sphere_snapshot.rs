use std::ops::{Add, Mul};
use crate::game_core::math::{FixedPoint, FixedPointV3};
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

#[derive(Copy, Clone, Debug, Default)]
pub struct SphereSnapshot {
    pub position: FixedPointV3,
    pub radius: FixedPoint,
}
impl Add for SphereSnapshot{
    type Output = SphereSnapshot;

    fn add(self, other: SphereSnapshot) -> SphereSnapshot {
        SphereSnapshot{
            position: self.position + other.position,
            radius: self.radius + other.radius,
        }
    }
}
impl Mul<FixedPoint> for SphereSnapshot{
    type Output = SphereSnapshot;

    fn mul(self, other: FixedPoint) -> SphereSnapshot {
        SphereSnapshot{
            position: self.position * other,
            radius: self.radius * other,
        }
    }
}
impl ViewSnapshot for SphereSnapshot{}