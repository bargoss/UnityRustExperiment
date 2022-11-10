use bevy_ecs;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Commands, Query, Res, ResMut, Schedule, SystemStage};
use bevy_ecs::schedule::Stage;
use bevy_ecs::world::World;
use bevy_math::Vec3;

pub struct Game{
    pub world: bevy_ecs::prelude::World,
    pub start_schedule: Schedule,
    pub update_schedule: Schedule,
    // big float array of 6000 floats
    pub positions_floats: Box<[f32; 6000]>,
}
// impl

impl Game {
    pub fn new() -> Game {
        let mut world = World::new();
        // this inserts the resource by its type i think, so we wont be able to have another resource of the same type
        // correct way to do this is probably to have a resource that is a hashmap of resources
        // or to create a new new resource type for each resource
        world.insert_resource(Vec::<f32>::new());

        let mut start_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        let mut create_bubble_points_stage = SystemStage::parallel();
        create_bubble_points_stage.add_system(create_bubble_points);
        start_schedule.add_stage("create_bubble_points", create_bubble_points_stage);

        let mut handle_bubble_velocities_stage = SystemStage::parallel();
        handle_bubble_velocities_stage.add_system(handle_bubble_velocities);
        update_schedule.add_stage("handle_bubble_velocities", handle_bubble_velocities_stage);

        let mut handle_bubble_forces_stage = SystemStage::parallel();
        handle_bubble_forces_stage.add_system(handle_bubble_forces);
        update_schedule.add_stage("handle_bubble_forces", handle_bubble_forces_stage);


        //update_position_views
        let mut update_position_views_stage = SystemStage::parallel();
        update_position_views_stage.add_system(update_position_views);
        update_schedule.add_stage("update_position_views", update_position_views_stage);

        Game {
            world,
            start_schedule,
            update_schedule,
            positions_floats: [0.0; 6000].into(),
        }
    }

    pub fn start(&mut self) {
        self.start_schedule.run(&mut self.world);
    }

    pub fn update(&mut self) {
        self.update_schedule.run(&mut self.world);
    }

    // get the Vec<Vec3> resource, return its iterator
    pub fn get_positions_iter(&mut self) -> impl Iterator<Item = &f32> {
        let resource = self.world.get_resource::<Vec<f32>>().unwrap();
        resource.iter()
    }
}


fn start_new_world() {
    // create an ecs world
    let mut world = bevy_ecs::prelude::World::new();

    // create a schedule
    let mut schedule = Schedule::default();

    // add systems to the schedule
    let initStage = schedule.add_stage("bubble_init_stage", SystemStage::parallel());

    schedule.add_system_to_stage("bubble_init_stage", create_bubble_points);


    // run the schedule
    schedule.run(&mut world);

}

#[derive(Component, Clone ,Debug, Default)]
pub struct BubblePoint {
    pub effect_radius: f32,
    pub target_distance: f32,
    pub velocity: Vec3,
    pub position: Vec3,
}
#[derive(Bundle, Clone, Default)]
pub struct BubblePointBundle {
    pub bubble_point: BubblePoint,
}
fn create_bubble_points(mut commands: Commands) {
    let bundle = BubblePointBundle {
        bubble_point: BubblePoint {
            position: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 0.0),
            effect_radius: 1.0,
            target_distance: 1.0,
        },
        ..Default::default()
    };

    // spawn 20 in random positions
    for _ in 0..2000 {
        let position = Vec3::new(
            rand::random::<f32>() * 10.0 - 5.0,
            rand::random::<f32>() * 10.0 - 5.0,
            0.0,
        );
        let mut bundle_clone = bundle.clone();
        bundle_clone.bubble_point.position = position;

        commands.spawn_bundle(bundle_clone);
    }
}
fn handle_bubble_velocities(mut query: Query<(&mut BubblePoint)>) {
    for mut bubble_point in query.iter_mut() {
        let vel = bubble_point.velocity.clone();
        bubble_point.position += vel;
        bubble_point.velocity *= 0.95;
    }
}
fn handle_bubble_forces(mut query: Query<(&mut BubblePoint)>,){
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut bp0, mut bp1]) = combinations.fetch_next() {
        let distance = bp0.position.distance(bp1.position);
        if distance < bp0.effect_radius {
            let direction = bp0.position - bp1.position;
            let force = direction.normalize() * (bp0.effect_radius - distance) * 0.01; //0.1;
            bp0.velocity += force;
            bp1.velocity -= force;
        }
    }
}

// also takes a Vec3 vector and writes the bubble_positions, which is a Vec<Vec3> in the game resource
fn update_position_views(mut query: Query<(&BubblePoint)>, mut bubble_positions: ResMut<Vec<f32>>) {
    bubble_positions.clear();
    for bubble_point in query.iter() {
        bubble_positions.push(bubble_point.position.x);
        bubble_positions.push(bubble_point.position.y);
        bubble_positions.push(bubble_point.position.z);
    }
}
