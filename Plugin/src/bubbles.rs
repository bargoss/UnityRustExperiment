use bevy_ecs;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Commands, Query, Res, ResMut, Schedule, SystemStage};
use bevy_ecs::schedule::Stage;
use bevy_ecs::world::World;
use bevy_math::Vec3;

const BUBBLE_COUNT: usize = 500;

pub struct Game{
    pub world: bevy_ecs::prelude::World,
    pub update_schedule: Schedule,
    pub positions_floats: Box<[f32; BUBBLE_COUNT * 3]>,
}
// impl

impl Game {
    pub fn new() -> Game {
        let mut world = World::new();
        // this inserts the resource by its type i think, so we wont be able to have another resource of the same type
        // correct way to do this is probably to have a resource that is a hashmap of resources
        // or to create a new new resource type for each resource
        world.insert_resource(Vec::<f32>::new()); // bubble positions for viewing
        world.insert_resource(BubblePushPoints{ points: Vec::new(), });

        let mut create_bubble_points_stage = SystemStage::parallel();
        create_bubble_points_stage.add_system(create_bubble_points);
        create_bubble_points_stage.run(&mut world);




        let mut update_schedule = Schedule::default();
        let mut handle_bubble_velocities_stage = SystemStage::parallel();
        handle_bubble_velocities_stage.add_system(handle_bubble_velocities);
        update_schedule.add_stage("handle_bubble_velocities", handle_bubble_velocities_stage);

        let mut handle_bubble_forces_stage = SystemStage::parallel();
        handle_bubble_forces_stage.add_system(handle_bubble_interactions);
        handle_bubble_forces_stage.add_system(handle_bubble_pull_to_center);
        handle_bubble_forces_stage.add_system(handle_bubble_push);
        update_schedule.add_stage("handle_bubble_forces", handle_bubble_forces_stage);



        //update_position_views
        let mut update_position_views_stage = SystemStage::parallel();
        update_position_views_stage.add_system(update_position_views);
        update_schedule.add_stage("update_position_views", update_position_views_stage);

        Game {
            world,
            update_schedule,
            positions_floats: [0.0; BUBBLE_COUNT*3].into(),
        }
    }


    pub fn update(&mut self) {
        self.update_schedule.run(&mut self.world);
    }

    // get the Vec<Vec3> resource, return its iterator
    pub fn get_positions_iter(&mut self) -> impl Iterator<Item = &f32> {
        let resource = self.world.get_resource::<Vec<f32>>().unwrap();
        resource.iter()
    }

    pub fn set_push_points(&mut self, points: Vec<Vec3>) {
        let mut resource = self.world.get_resource_mut::<BubblePushPoints>().unwrap();
        resource.points.clear();

        for point in points {
            resource.points.push(point);
        }
    }
}


// resource with a Vec<Vec3> for bubble push points
struct BubblePushPoints {
    points: Vec<Vec3>,
}


#[derive(Component, Clone ,Debug, Default)]
pub struct Bubble {
    pub effect_radius: f32,
    pub target_distance: f32
}
#[derive(Component, Clone ,Debug, Default)]
pub struct Position {
    pub value: Vec3
}
#[derive(Component, Clone ,Debug, Default)]
pub struct Velocity {
    pub value: Vec3
}


#[derive(Bundle, Clone, Default)]
pub struct BubblePointBundle {
    pub bubble: Bubble,
    pub position: Position,
    pub velocity: Velocity,
}

pub struct CreateBubblePointsParams {
    pub count: usize,
}

fn create_bubble_points(mut commands: Commands) {
    let bundle = BubblePointBundle {
        bubble: Bubble {
            effect_radius: 1.0,
            target_distance: 1.0,
        },
        ..Default::default()
    };

    // spawn 20 in random positions
    for _ in 0..BUBBLE_COUNT {
        let position = Vec3::new(
            rand::random::<f32>() * 10.0 - 5.0,
            rand::random::<f32>() * 10.0 - 5.0,
            0.0,
        );
        let mut bundle_clone = bundle.clone();
        bundle_clone.position.value = position;

        commands.spawn_bundle(bundle_clone);
    }
}

fn handle_bubble_velocities(mut query: Query<(&mut Position, &mut Velocity)>) {
    for (mut position, mut velocity) in query.iter_mut() {
        position.value += velocity.value;
        velocity.value *= 0.95;
    }
}
fn handle_bubble_interactions(mut query: Query<(&Bubble, &Position, &mut Velocity)>,){
    let mut combinations = query.iter_combinations_mut();
    while let Some([(bubble1, position1, mut velocity1), (bubble2, position2, mut velocity2)]) = combinations.fetch_next() {
        let distance = position1.value.distance(position2.value);
        let effect_radius = bubble1.effect_radius + bubble2.effect_radius;
        if distance < effect_radius {
            let force = (effect_radius - distance) / effect_radius;
            let direction = (position1.value - position2.value).normalize();
            velocity1.value += direction * force;
            velocity2.value -= direction * force;
        }
    }
}

fn handle_bubble_pull_to_center(mut query: Query<(&Position, &mut Velocity)>){
    for (position, mut velocity) in query.iter_mut() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let delta_to_center = center - position.value;
        let direction = delta_to_center.normalize();
        let force = 1.5;
        velocity.value += direction * force;
    }
}

fn handle_bubble_push(mut query: Query<(&Position, &mut Velocity)>, push_points: Res<BubblePushPoints>){
    for (position, mut velocity) in query.iter_mut() {
        for push_point in push_points.points.iter() {
            let delta_to_push_point = *push_point - position.value;
            let sqr_distance = delta_to_push_point.length_squared();
            if sqr_distance < 2.0 {
                let direction = delta_to_push_point.normalize();
                let force = 2.0 - sqr_distance;
                velocity.value += direction * force;
            }
        }
    }
}



//fn handle_bubble_forces2(mut query: Query<(&mut BubblePoint)>,){
//    let mut combinations = query.iter_combinations_mut();
//    while let Some([mut bp0, mut bp1]) = combinations.fetch_next() {
//        let distance = bp0.position.distance(bp1.position);
//        if distance < bp0.effect_radius {
//            let direction = bp0.position - bp1.position;
//            let force = direction.normalize() * (bp0.effect_radius - distance) * 0.01; //0.1;
//            bp0.velocity += force;
//            bp1.velocity -= force;
//        }
//    }
//}

// also takes a Vec3 vector and writes the bubble_positions, which is a Vec<Vec3> in the game resource
fn update_position_views(mut query: Query<(&Position)>, mut bubble_positions: ResMut<Vec<f32>>) {
    bubble_positions.clear();
    for bubble_point in query.iter() {
        bubble_positions.push(bubble_point.value.x);
        bubble_positions.push(bubble_point.value.y);
        bubble_positions.push(bubble_point.value.z);
    }
}
