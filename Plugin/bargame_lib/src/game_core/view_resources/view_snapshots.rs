use crate::game_core::common::Vector3;

pub struct LineSnapshot {
    pub start: Vector3,
    pub end: Vector3,
    pub width: f32,
}

pub struct SphereSnapshot {
    pub position: Vector3,
    pub radius: f32,
}

pub struct LineSnapshots{
    pub lines: Vec<LineSnapshot>,
}

pub struct SphereSnapshots{
    pub spheres: Vec<SphereSnapshot>,
}