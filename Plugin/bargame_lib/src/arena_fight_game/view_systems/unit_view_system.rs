use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::{BelongsToFaction, Faction, UnitView};
use crate::game_core::components::CircleCollider;
use crate::game_core::components::position::Position;
use crate::game_core::resources::time::Time;
use crate::game_core::verlet_physics::FixedPoint;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::sphere_snapshot::SphereSnapshot;

fn faction_to_color(faction: &Faction) -> [f32; 4] {
    match faction {
        Faction::Neutral => [1.0, 1.0, 1.0, 1.0],
        Faction::Blue => [0.0, 0.0, 1.0, 1.0],
        Faction::Red => [1.0, 0.0, 0.0, 1.0],
        Faction::Green => [0.0, 1.0, 0.0, 1.0],
        Faction::Yellow => [1.0, 1.0, 0.0, 1.0],
    }
}

pub fn unit_view_system(
    unit_query: Query<(&UnitView, &Position, &CircleCollider, &BelongsToFaction)>,
    mut sphere_snapshots: ResMut<BufferedViewSnapshotInterpolator<SphereSnapshot>>,
    time: Res<Time>
) {
    for (unit_view, position, circle_collider, belongs_to_faction) in unit_query.iter() {
        let time = FixedPoint::new(time.tick as f64) * time.fixed_delta_time;
        let position = position.value;
        let radius = circle_collider.radius;

        // put them in one line
        //println!("view_time: {}, view_position: {}, radius: {}", time, position, radius);

        sphere_snapshots.push(unit_view.view_custom_id, time, SphereSnapshot{
            position : position.into(),
            radius : circle_collider.radius,
            color: faction_to_color(&belongs_to_faction.faction)
        });
    }
}