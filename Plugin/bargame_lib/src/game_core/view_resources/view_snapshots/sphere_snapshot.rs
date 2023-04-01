use std::ops::{Add, Mul};
use crate::game_core::math::{FixedPoint, FixedPointV3};
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

#[derive(Copy, Clone, Debug, Default)]
pub struct SphereSnapshot {
    pub position: FixedPointV3,
    pub radius: FixedPoint,
    pub color: [f32; 4],
}
impl Add for SphereSnapshot{
    type Output = SphereSnapshot;

    fn add(self, other: SphereSnapshot) -> SphereSnapshot {
        SphereSnapshot{
            position: self.position + other.position,
            radius: self.radius + other.radius,
            color: [
                self.color[0] + other.color[0],
                self.color[1] + other.color[1],
                self.color[2] + other.color[2],
                self.color[3] + other.color[3],
            ]
        }
    }
}
impl Mul<FixedPoint> for SphereSnapshot{
    type Output = SphereSnapshot;

    fn mul(self, other: FixedPoint) -> SphereSnapshot {
        SphereSnapshot{
            position: self.position * other,
            radius: self.radius * other,
            color: [
                self.color[0] * other.to_f32(),
                self.color[1] * other.to_f32(),
                self.color[2] * other.to_f32(),
                self.color[3] * other.to_f32(),
            ]
        }
    }
}
impl ViewSnapshot for SphereSnapshot{}