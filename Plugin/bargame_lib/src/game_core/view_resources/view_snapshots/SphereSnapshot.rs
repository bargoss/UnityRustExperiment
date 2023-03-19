use std::ops::{Add, Mul};
use crate::game_core::common::Vector3;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

#[derive(Copy, Clone, Debug, Default)]
pub struct SphereSnapshot {
    pub position: Vector3,
    pub radius: f32,
}
impl Add for SphereSnapshot{
    type Output = SphereSnapshot;

    fn add(self, other: SphereSnapshot) -> SphereSnapshot {
        SphereSnapshot{
            position: self.position + other.position,
            radius: self.radius,
        }
    }
}
impl Mul<f32> for SphereSnapshot{
    type Output = SphereSnapshot;

    fn mul(self, other: f32) -> SphereSnapshot {
        SphereSnapshot{
            position: self.position * other,
            radius: self.radius,
        }
    }
}
impl ViewSnapshot for SphereSnapshot{}