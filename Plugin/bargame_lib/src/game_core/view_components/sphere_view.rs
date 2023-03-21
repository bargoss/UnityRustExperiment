use super::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SphereView {
    pub radius: FixedPoint,
    pub view_custom_id: Id
}