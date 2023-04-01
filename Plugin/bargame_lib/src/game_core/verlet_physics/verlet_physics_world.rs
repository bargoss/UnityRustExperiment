use std::collections::{BTreeMap};
use bevy_ecs::prelude::Resource;
use crate::game_core::common::id::Id;
use crate::game_core::verlet_physics::verlet_beam::VerletBeam;
use crate::game_core::verlet_physics::verlet_object::VerletObject;
use super::*;


pub struct Entry<TVal> {
    pub val: TVal,
}

#[derive(Resource)]
pub struct VerletPhysicsWorld {
    objects: BTreeMap<Id, Entry<VerletObject>>,
    beams: BTreeMap<Id, Entry<VerletBeam>>,
    spatial_partitioning: SpacialPartitioning<20>,
    overlap_circle_buffer: Vec<u32>,
    iteration_id_buffer: Vec<u32>,
}

impl Default for VerletPhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl VerletPhysicsWorld {
    pub fn new() -> VerletPhysicsWorld {
        VerletPhysicsWorld {
            objects: BTreeMap::new(),
            beams: BTreeMap::new(),
            spatial_partitioning: SpacialPartitioning::<20>::new(FixedPoint::new(5.0)),
            overlap_circle_buffer: Vec::new(),
            iteration_id_buffer: Vec::new(),
        }
    }

    // handle the dirty objects
    fn sync_objects_and_beams(&mut self) {
        self.spatial_partitioning.clear();

        for (id, entry) in self.objects.iter_mut() {
            self.spatial_partitioning.add_circle(id.0, entry.val.position, entry.val.radius);
        }
    }

    pub fn solve_verlet_collision(object1: &mut VerletObject, object2: &mut VerletObject, min_dist: FixedPoint, response_coef: FixedPoint) {
        let v = object1.position - object2.position;
        let dist = v.magnitude();

        let n = if dist == FixedPoint::zero() { FixedPointV2::zero() } else { v / dist };
        let mass_ratio_1 = object1.mass / (object1.mass + object2.mass);
        let mass_ratio_2 = object2.mass / (object1.mass + object2.mass);
        let delta = FixedPoint::new(0.5) * response_coef * (dist - min_dist);
        let obj1_translation = -n * (mass_ratio_2 * delta);
        let obj2_translation = n * (mass_ratio_1 * delta);

        object1.position += obj1_translation;
        object2.position += obj2_translation;
    }

    pub fn overlap_circle(&self, position: FixedPointV2, radius: FixedPoint, overlap_circle_buffer: &mut Vec<u32>) {
        self.spatial_partitioning.overlap_circle(position, radius, overlap_circle_buffer);

        // sqr mag check and remove
        overlap_circle_buffer.retain(|id| {
            let obj = self.objects.get(&Id(*id)).unwrap().val;
            let delta = obj.position - position;
            let sqr_dist = delta.magnitude_squared();

            let collision_dist_squared = (radius + obj.radius) * (radius + obj.radius);

            sqr_dist < collision_dist_squared
        });
    }

    fn solve_object_collisions(&mut self, iteration_id_buffer: &mut Vec<u32>, overlap_circle_buffer: &mut Vec<u32>) {
        iteration_id_buffer.clear();
        self.objects
            .iter()
            .filter(|(_, entry)| !entry.val.is_static)
            .for_each(|(id, _)| iteration_id_buffer.push(id.0));

        // iterate non-static
        for id0 in iteration_id_buffer.iter() {
            let mut obj0 = self.objects.get(&Id(*id0)).unwrap().val;

            // get possible neighbours
            self.overlap_circle(obj0.position, obj0.radius, overlap_circle_buffer);

            // check for collisions
            for other_id in overlap_circle_buffer.iter().filter(|id| **id != *id0) {
                let mut obj1 = self.objects.get(&Id(*other_id)).unwrap().val;

                let min_dist = obj0.radius + obj1.radius;
                VerletPhysicsWorld::solve_verlet_collision(&mut obj0, &mut obj1, min_dist, FixedPoint::new(0.75));

                if !obj1.is_static {
                    self.add_or_set_object(obj1, Id(*other_id));
                }
            }
            self.add_or_set_object(obj0, Id(*id0));
        }
    }

    fn solve_beams(&mut self) {
        let beam_keys = self.beams.keys().cloned().collect::<Vec<_>>(); // todo: optimize this via not allocating a new vec every frame
        for i in 0..beam_keys.len() {
            let beam_key = beam_keys[i];
            let beam = self.beams.get(&beam_key).unwrap().val;
            let id_a = beam.verlet_object_id_a;
            let id_b = beam.verlet_object_id_b;

            if let Some(entry_a) = self.objects.get(&id_a) {
                if let Some(entry_b) = self.objects.get(&id_b) {
                    let mut obj0 = entry_a.val.clone();
                    let mut obj1 = entry_b.val.clone();

                    let min_dist = beam.length;
                    VerletPhysicsWorld::solve_verlet_collision(&mut obj0, &mut obj1, min_dist, FixedPoint::new(0.75));

                    if !obj0.is_static {
                        self.add_or_set_object(obj0, id_a);
                    }
                    if !obj1.is_static {
                        self.add_or_set_object(obj1, id_b);
                    }
                }
            }
        }
    }

    fn update_objects(&mut self, dt: FixedPoint) {
        /*
            for (var i = 0; i < m_Objects.Count; i++)
            {
                var obj = m_Objects[i];
                var displacement = obj.Position - obj.PositionLast;
                obj.PositionLast = obj.Position;
                obj.Position += displacement + obj.Acceleration * (dt * dt);
                obj.Acceleration = FixedPointVector2.Zero;

                m_Objects[i] = obj;
            }
        */

        let mut iteration_id_buffer = Vec::new();
        self.objects
            .iter()
            .filter(|(_, entry)| !entry.val.is_static)
            .for_each(|(id, _)| iteration_id_buffer.push(id.0));

        for id in iteration_id_buffer.iter() {
            let mut obj = self.objects.get(&Id(*id)).unwrap().val;

            let displacement = obj.position - obj.position_last;
            obj.position_last = obj.position;
            obj.position += displacement + obj.acceleration * (dt * dt);
            obj.acceleration = FixedPointV2::zero();

            self.add_or_set_object(obj, Id(*id));
        }




    }

    pub fn update(&mut self, dt: FixedPoint, iteration_id_buffer: &mut Vec<u32>, overlap_circle_buffer: &mut Vec<u32>) {
        let steps = 2;
        let _step_dt = dt / FixedPoint::new(steps as f64);


        self.sync_objects_and_beams();

        for _ in 0..steps {
            self.solve_object_collisions(iteration_id_buffer, overlap_circle_buffer);
            self.solve_beams();
        }
        self.update_objects(dt);
        self.sync_objects_and_beams();
    }

    pub fn get_obj_iter(&self) -> impl Iterator<Item = &VerletObject> {
        self.objects.iter().map(|(_, entry)| &entry.val)
    }

    pub fn get_obj_iter_mut(&mut self) -> impl Iterator<Item = &mut VerletObject> {
        self.objects.iter_mut().map(|(_, entry)| &mut entry.val)
    }

    pub fn get_beam_iter(&self) -> impl Iterator<Item = &VerletBeam> {
        self.beams.iter().map(|(_, entry)| &entry.val)
    }

    pub fn set_object(&mut self, object : VerletObject, id : Id) {
        self.objects.insert(id, Entry{val: object});
    }
    pub fn add_or_set_object(&mut self, object : VerletObject, id : Id) {
        self.objects.insert(id, Entry{val: object});
    }

    pub fn remove_object(&mut self, id : Id) {
        self.objects.remove(&id);
        self.spatial_partitioning.remove_with_id(id.0);
    }

    pub fn add_beam(&mut self, beam : VerletBeam, id : Id) {
        self.beams.insert(id, Entry{val: beam});
    }

    pub fn add_or_set_beam(&mut self, beam : VerletBeam, id : Id) {
        self.beams.insert(id, Entry{val: beam});
    }

    pub fn remove_beam(&mut self, id : Id) {
        self.beams.remove(&id);
    }



    pub fn get_object(&self, id : Id) -> Option<VerletObject> {
        self.objects.get(&id).map(|entry| entry.val)
    }

    pub fn get_beam(&self, id : Id) -> Option<VerletBeam> {
        self.beams.get(&id).map(|entry| entry.val)
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.beams.clear();
        self.spatial_partitioning.clear();
    }
}