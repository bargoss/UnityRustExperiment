use bargame_lib::game_core::math::{FP, FP2, FP3};
use ggez::glam::{Vec2, Vec3};

pub fn to_glam_vec2(vec: FP2) -> Vec2 {
    let x = vec.x().to_f32();
    let y = vec.y().to_f32();
    Vec2::new(x, y)
}

pub fn to_glam_vec3(vec: FP3) -> Vec3 {
    let x = vec.x().to_f32();
    let y = vec.y().to_f32();
    let z = vec.z().to_f32();
    Vec3::new(x, y, z)
}