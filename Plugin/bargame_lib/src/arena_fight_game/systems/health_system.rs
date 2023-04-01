use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::*;
use crate::game_core::math::FP;
use crate::game_core::resources::*;

pub fn health_system(
    mut commands: Commands,
    mut health_query: Query<(&mut Health, Entity)>,
    time: Res<Time>,
) {
    let dt = time.fixed_delta_time;

    for (mut health,entity) in health_query.iter_mut() {
        if health.health <= FP::zero() {
            commands.entity(entity).despawn();
            continue;
        }

        let change = health.health_regen_per_second * dt;
        health.health += change;
        health.health = health.health.clamp(FP::zero(), health.max_health);
    }
}