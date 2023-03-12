use bargame_lib::game_core::math::{FixedPoint, FixedPointV2, FixedPointV3};
use ggez::glam::{Vec2, Vec3};

pub fn to_glam_vec2(vec: FixedPointV2) -> Vec2 {
    let x = FixedPoint(vec.0.x).to_f32();
    let y = FixedPoint(vec.0.y).to_f32();
    Vec2::new(x, y)
}

pub fn to_glam_vec3(vec: FixedPointV3) -> Vec3 {
    let x = FixedPoint(vec.0.x).to_f32();
    let y = FixedPoint(vec.0.y).to_f32();
    let z = FixedPoint(vec.0.z).to_f32();
    Vec3::new(x, y, z)
}