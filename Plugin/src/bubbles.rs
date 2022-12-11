mod spatial_ds;

use bevy_ecs;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, Query, Res, ResMut, Schedule, SystemStage};
use bevy_ecs::schedule::Stage;
use bevy_ecs::world::World;
use bevy_math::Vec3;
use crate::bubbles::spatial_ds::LookUpGrids;

//const BUBBLE_COUNT: usize = 500;
const DELTA_TIME: f32 = 0.5;
const POSITION_FLOAT_BUFFER_SIZE: usize = 10000*3;
// use macro
//const BUBBLE_COUNT_3: usize = 1500;

pub struct Game{
    pub world: bevy_ecs::prelude::World,
    pub update_schedule: Schedule,
}
// impl

pub struct PositionFloatBuffer{
    pub value: [f32; POSITION_FLOAT_BUFFER_SIZE]
}

pub struct WorldParams{
    pub bubble_count: usize,
}

impl Game {
    pub fn new(world_params: WorldParams) -> Game {
        let mut world = World::new();
        // this inserts the resource by its type i think, so we wont be able to have another resource of the same type
        // correct way to do this is probably to have a resource that is a hashmap of resources
        // or to create a new new resource type for each resource
        // f32 array that is in a box

        // create PositionFloatBuffer instance
        world.insert_resource(world_params);
        let position_float_buffer = PositionFloatBuffer{ value: [0.0; POSITION_FLOAT_BUFFER_SIZE] };
        world.insert_resource(BubblePushPoints{ points: Vec::new()});
        world.insert_resource(position_float_buffer);
        let lookup_grids = LookUpGrids::<u32>::new(3.0);
        world.insert_resource(lookup_grids);
        world.insert_resource(Vec::<(u32, u32)>::new()); // buffer for iterating over neighbor pair ids

        let mut create_bubble_points_stage = SystemStage::single_threaded();
        create_bubble_points_stage.add_system(create_bubble_points);
        create_bubble_points_stage.run(&mut world);




        let mut update_schedule = Schedule::default();

        let mut pre_update_stage = SystemStage::single_threaded();
        pre_update_stage.add_system(update_lookup_grids);
        update_schedule.add_stage("pre_update", pre_update_stage);

        let mut update_bubble_velocities_stage = SystemStage::single_threaded();
        update_bubble_velocities_stage.add_system(handle_bubble_interactions);
        update_bubble_velocities_stage.add_system(handle_bubble_pull_to_center);
        update_bubble_velocities_stage.add_system(handle_bubble_push);
        update_schedule.add_stage("handle_bubble_velocities", update_bubble_velocities_stage);

        let mut update_bubble_positions_stage = SystemStage::single_threaded();
        update_bubble_positions_stage.add_system(handle_bubble_velocities);
        update_bubble_positions_stage.add_system(update_position_views);
        update_schedule.add_stage("handle_bubble_positions", update_bubble_positions_stage);


        Game {
            world,
            update_schedule,
        }
    }


    pub fn update(&mut self) {
        self.update_schedule.run(&mut self.world);
    }


    pub fn get_positions_arr(&mut self) ->  [f32; POSITION_FLOAT_BUFFER_SIZE] {
        let resource = self.world.get_resource::<PositionFloatBuffer>().unwrap();
        resource.value
    }

    pub fn get_positions_iter(&mut self) -> impl Iterator<Item = &f32> {
        let resource = self.world.get_resource::<PositionFloatBuffer>().unwrap();
        resource.value.iter()
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
pub struct BubblePushPoints {
    pub points: Vec<Vec3>,
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



fn create_bubble_points(mut commands: Commands, world_params: Res<WorldParams>) {
    let bundle = BubblePointBundle {
        bubble: Bubble {
            effect_radius: 1.0,
            target_distance: 1.0,
        },
        ..Default::default()
    };

    // spawn 20 in random positions
    for _ in 0..world_params.bubble_count {
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
        velocity.value *= 0.95 * DELTA_TIME;
    }
}

// a system that iterates entities with their id
fn update_lookup_grids(mut query: Query<(&Position, Entity)>, mut lookup_grids: ResMut<LookUpGrids<u32>>) {
    lookup_grids.clear();
    for (position, entity) in query.iter_mut() {
        lookup_grids.add(entity.id(), position.value);
    }
}



// query for
fn handle_bubble_interactions(
    mut read_query: Query<(&Bubble, &Position)>,
    mut write_query : Query<(&Bubble, &Position, &mut Velocity)>,
    lookup_grids: Res<LookUpGrids<u32>>,
    mut buffer: ResMut<Vec::<(u32, u32)>>, // for neighbor pair ids
    world_params: Res<WorldParams>,
) {
    lookup_grids.get_all_neighbours(&mut buffer);

    for (id_a, id_b) in buffer.iter(){
        let entity_a = Entity::from_raw(id_a.clone());
        let entity_b = Entity::from_raw(id_b.clone());

        let (bubble_a, position_a) = read_query.get(entity_a).unwrap();
        let (bubble_b, position_b) = read_query.get(entity_b).unwrap();

        let force = calculate_neighbour_force(
            position_a.value,
            position_b.value,
            bubble_a,
            bubble_b,
        );


        let mut velocity_a = write_query.get_mut(entity_a).unwrap().2;
        velocity_a.value += force;
        let mut velocity_b = write_query.get_mut(entity_b).unwrap().2;
        velocity_b.value -= force;
    }
}

fn calculate_neighbour_force(
    pos_a: Vec3, pos_b: Vec3,
    bubble_a : &Bubble,
    bubble_b : &Bubble,

) -> Vec3 {
    let distance_sqr = pos_a.distance_squared(pos_b);
    let effect_radius = bubble_a.effect_radius + bubble_b.effect_radius;
    let effect_radius_sqr = effect_radius * effect_radius;

    if distance_sqr < effect_radius_sqr {
        let distance = pos_a.distance(pos_b);
        let direction = (pos_a - pos_b).normalize();
        let force = (effect_radius - distance) * 0.1 * 1.0;
        return direction * force;
    }
    return Vec3::ZERO;
}

//use get_many_mut not to fight the borrow checker
fn handle_bubble_interactions3(mut query: Query<(&Bubble, &Position, &mut Velocity)>, lookup_grids: Res<LookUpGrids<u32>>) {
    let a = query.get_mut(Entity::from_raw(0));
    for (bubble1, position1, mut velocity1) in query.get_mut(Entity::from_raw(0)){

    }
}


fn handle_bubble_interactions2(mut query: Query<(&Bubble, &Position, &mut Velocity)>, lookup_grids: Res<LookUpGrids<usize>>){
    for (bubble1, position1, mut velocity1) in query.iter_mut() {}

    let mut combinations = query.iter_combinations_mut();
    while let Some([(bubble1, position1, mut velocity1), (bubble2, position2, mut velocity2)]) = combinations.fetch_next() {
        let distance = position1.value.distance(position2.value);
        let effect_radius = bubble1.effect_radius + bubble2.effect_radius;
        if distance < effect_radius {
            let force = (effect_radius - distance) / effect_radius;
            let direction = (position1.value - position2.value).normalize();
            velocity1.value += direction * force * DELTA_TIME;
            velocity2.value -= direction * force * DELTA_TIME;
        }
    }
}

fn handle_bubble_pull_to_center(mut query: Query<(&Position, &mut Velocity)>){
    for (position, mut velocity) in query.iter_mut() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let delta_to_center = center - position.value;
        let direction = delta_to_center.normalize();
        let force = 0.05;
        velocity.value += direction * force * DELTA_TIME;
    }
}

fn handle_bubble_push(mut query: Query<(&Position, &mut Velocity)>, mut push_points: ResMut<BubblePushPoints>){
    for (position, mut velocity) in query.iter_mut() {
        for push_point in push_points.points.iter() {
            let delta_to_push_point = position.value - *push_point;
            let sqr_distance = delta_to_push_point.length_squared();
            let effect_radius = 15.0;
            if sqr_distance < effect_radius * effect_radius {
                let direction = delta_to_push_point.normalize();
                velocity.value += direction * 0.2 * DELTA_TIME;
            }
        }
    }

    // clear the push points
    push_points.points.clear();
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

fn update_position_views(mut query: Query<&Position>, mut bubble_positions: ResMut<PositionFloatBuffer>) {
    for (i, position) in query.iter_mut().enumerate() {
        bubble_positions.value[i*3] = position.value.x;
        bubble_positions.value[i*3+1] = position.value.y;
        bubble_positions.value[i*3+2] = position.value.z;
    }
}
