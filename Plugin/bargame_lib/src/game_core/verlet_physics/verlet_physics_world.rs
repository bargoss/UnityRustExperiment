use std::collections::{BTreeMap, HashMap};
use crate::game_core::verlet_physics::verlet_beam::VerletBeam;
use crate::game_core::verlet_physics::verlet_object::VerletObject;
use super::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Index(u32);

impl Index {
    pub fn new(p0: u32) -> Self {
        Index(p0)
    }
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Id(u32);

impl Id {
    pub fn new(p0: u32) -> Self {
        Id(p0)
    }
}

pub struct Entry<TVal> {
    pub val: TVal,
    pub out_of_sync: bool,
}

pub struct VerletPhysicsWorld {
    objects: BTreeMap<Id, Entry<VerletObject>>,
    beams: BTreeMap<Id, Entry<VerletBeam>>,
    spatial_partitioning: SpacialPartitioning<20>,
    overlap_circle_buffer: Vec<u32>,
    iteration_id_buffer: Vec<u32>,
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
        for (id, entry) in self.objects.iter_mut() {
            if entry.out_of_sync {
                entry.out_of_sync = false;
            }

            self.spatial_partitioning.add_circle(id.0, entry.val.position, entry.val.radius);
        }

        for (id, entry) in self.beams.iter_mut() {
            if entry.out_of_sync {
                entry.out_of_sync = false;
            }
        }
    }

    /*
        private void SolveDistanceConstraint(VerletObject object1, VerletObject object2, FixedPoint minDist, FixedPoint responseCoef, out FixedPointVector2 obj1Translation, out FixedPointVector2 obj2Translation)
        {
            var v = object1.Position - object2.Position;
            var dist = v.Magnitude;
            var n = v / dist;
            var massRatio1 = object1.Radius / (object1.Radius + object2.Radius);
            var massRatio2 = object2.Radius / (object1.Radius + object2.Radius);
            var delta = (FixedPoint)0.5f * responseCoef * (dist - minDist);
            obj1Translation = -n * (massRatio2 * delta);
            obj2Translation = n * (massRatio1 * delta);
        }
    */

    pub fn solve_verlet_collision(object1: &mut VerletObject, object2: &mut VerletObject, min_dist: FixedPoint, response_coef: FixedPoint) {
        let v = *object1.position - *object2.position;
        let dist = v.magnitude();
        let n = v / dist;
        let mass_ratio_1 = *object1.mass / (*object1.mass + *object2.mass);
        let mass_ratio_2 = *object2.mass / (*object1.mass + *object2.mass);
        let delta = *FixedPoint::new(0.5) * *response_coef * (dist - *min_dist);
        let obj1_translation = -n * (mass_ratio_2 * delta);
        let obj2_translation = n * (mass_ratio_1 * delta);

        *object1.position += obj1_translation;
        *object2.position += obj2_translation;
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
            self.spatial_partitioning.overlap_circle(obj0.position, obj0.radius, overlap_circle_buffer);

            // check for collisions
            for other_id in overlap_circle_buffer.iter() { // 000000000000000000
                let mut obj1 = self.objects.get(&Id(*other_id)).unwrap().val;

                let min_dist = FixedPoint(*obj0.radius + *obj1.radius);
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
            let id_A = Id(beam.verlet_object_id_a);
            let id_B = Id(beam.verlet_object_id_b);

            if let Some(entry_A) = self.objects.get(&id_A) {
                if let Some(entry_B) = self.objects.get(&id_B) {
                    let mut obj0 = entry_A.val.clone();
                    let mut obj1 = entry_B.val.clone();

                    let min_dist = FixedPoint(*beam.length);
                    VerletPhysicsWorld::solve_verlet_collision(&mut obj0, &mut obj1, min_dist, FixedPoint::new(0.75));

                    if !obj0.is_static {
                        self.add_or_set_object(obj0, id_A);
                    }
                    if !obj1.is_static {
                        self.add_or_set_object(obj1, id_B);
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

        for (_, entry) in self.objects.iter_mut().filter(|(_, entry)| !entry.val.is_static) {
            let mut obj = entry.val;

            let displacement = *obj.position - *obj.position_last;
            *obj.position_last = *obj.position;
            *obj.position += displacement + *obj.acceleration * (*dt * *dt);
            *obj.acceleration = *FixedPointV2::zero();
            entry.val = obj;
            entry.out_of_sync = true;
        }
    }

    pub fn update(&mut self, dt: FixedPoint, iteration_id_buffer: &mut Vec<u32>, overlap_circle_buffer: &mut Vec<u32>) {
        let steps = 2;
        let step_dt = FixedPoint(*dt / *FixedPoint::new(steps as f64));


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

    pub fn set_object(&mut self, object : VerletObject, id : Id) {
        self.objects.insert(id, Entry{val: object, out_of_sync: true});
    }
    pub fn add_or_set_object(&mut self, object : VerletObject, id : Id) {
        self.objects.insert(id, Entry{val: object, out_of_sync: true});
    }

    pub fn remove_object(&mut self, id : Id) {
        self.objects.remove(&id);
        self.spatial_partitioning.remove_with_id(id.0);
    }

    pub fn add_beam(&mut self, beam : VerletBeam, id : Id) {
        self.beams.insert(id, Entry{val: beam, out_of_sync: true});
    }

    pub fn add_or_set_beam(&mut self, beam : VerletBeam, id : Id) {
        self.beams.insert(id, Entry{val: beam, out_of_sync: true});
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
}