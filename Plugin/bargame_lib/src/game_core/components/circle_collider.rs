use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct CircleCollider {
    pub radius: FixedPoint
}