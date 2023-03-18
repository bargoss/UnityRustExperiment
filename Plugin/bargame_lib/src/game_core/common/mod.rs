use crate::game_core::math::{FixedPoint, FixedPointV2, FixedPointV3};

pub mod id;
pub mod index;

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

pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
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