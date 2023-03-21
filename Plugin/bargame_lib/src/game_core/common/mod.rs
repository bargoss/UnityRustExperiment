use crate::game_core::math::{FixedPoint, FixedPointV2, FixedPointV3};

pub mod id;
pub mod index;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2{
    pub x: f32,
    pub y: f32,
}


// make FixedPointV2 castable to Vector2
impl From<FixedPointV2> for Vector2 {
    fn from(fixed_point_v2: FixedPointV2) -> Self {
        Vector2 {
            x: fixed_point_v2.x().to_f32(),
            y: fixed_point_v2.y().to_f32(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// impl add for Vector3
impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// impl Mul<f32>
impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Vector3 {
    pub(crate) fn new(p0: f32, p1: f32, p2: f32) -> Vector3 {
        Vector3 {
            x: p0,
            y: p1,
            z: p2,
        }
    }
}

// make FixedPointV3 castable to Vector3
impl From<FixedPointV3> for Vector3 {
    fn from(fixed_point_v3: FixedPointV3) -> Self {
        Vector3 {
            x: fixed_point_v3.x().to_f32(),
            y: fixed_point_v3.y().to_f32(),
            z: fixed_point_v3.z().to_f32(),
        }
    }
}

// make FixedPointV2 castable to Vector3
impl From<FixedPointV2> for Vector3 {
    fn from(fixed_point_v2: FixedPointV2) -> Self {
        Vector3 {
            x: fixed_point_v2.x().to_f32(),
            y: fixed_point_v2.y().to_f32(),
            z: 0.0,
        }
    }
}