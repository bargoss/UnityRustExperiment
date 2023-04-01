use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::{BelongsToFaction, Faction, Health, UnitView};
use crate::game_core::components::CircleCollider;
use crate::game_core::components::position::Position;
use crate::game_core::resources::time::Time;
use crate::game_core::verlet_physics::FixedPoint;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::sphere_snapshot::SphereSnapshot;
use crate::game_core::common::Id;
use crate::game_core::math::FixedPointV3;

fn faction_to_color(faction: &Faction) -> [f32; 4] {
    match faction {
        Faction::Neutral => [1.0, 1.0, 1.0, 1.0],
        Faction::Blue => [0.1, 0.1, 1.0, 1.0],
        Faction::Red => [1.0, 0.1, 0.1, 1.0],
        Faction::Green => [0.1, 1.0, 0.1, 1.0],
        Faction::Yellow => [1.0, 1.0, 0.1, 1.0],
    }
}

pub fn unit_view_system(
    unit_query: Query<(&UnitView, &Position, &CircleCollider, &BelongsToFaction, Option<&Health>)>,
    mut sphere_snapshots: ResMut<BufferedViewSnapshotInterpolator<SphereSnapshot>>,
    time: Res<Time>
) {
    for (unit_view, position, circle_collider, belongs_to_faction, health_opt) in unit_query.iter() {
        let time = FixedPoint::new(time.tick as f64) * time.fixed_delta_time;
        let position = position.value;
        let radius = circle_collider.radius;

        if let Some(health) = health_opt {
            let health_ratio = FixedPoint::one() - health.health / health.max_health;
            let health_radius = radius * health_ratio;

            let position_v3 : FixedPointV3 = position.into();

            let more_smaller_prime = 100003;
            let black_circle_custom_view_id = Id::new(unit_view.view_custom_id.0*more_smaller_prime + 1);
            //sphere_snapshots.push(black_circle_custom_view_id, time, SphereSnapshot{
            //    position : position_v3 + FixedPointV3::from_num(0.0,0.0,-1.0),
            //    radius : health_radius,
            //    color: [0.1, 0.1, 0.1, 1.0]
            //});
            sphere_snapshots.push(unit_view.view_custom_id, time, SphereSnapshot{
                position : position.into(),
                radius : circle_collider.radius * (FixedPoint::one() - health_ratio),
                color: faction_to_color(&belongs_to_faction.faction)
            });
        }
        else{
            sphere_snapshots.push(unit_view.view_custom_id, time, SphereSnapshot{
                position : position.into(),
                radius : circle_collider.radius,
                color: faction_to_color(&belongs_to_faction.faction)
            });
        }
    }
}