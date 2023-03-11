use std::collections::{BTreeMap, HashMap};
use crate::game_core::verlet_physics::verlet_beam::VerletBeam;
use crate::game_core::verlet_physics::verlet_object::VerletObject;
use super::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Index(u32);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Id(u32);

pub struct Entry<TVal> {
    pub val: TVal,
    pub is_dirty: bool,
}

pub struct VerletPhysicsWorld {
    objects: BTreeMap<Id, Entry<VerletObject>>,
    beams: BTreeMap<Id, Entry<VerletBeam>>,
    spatial_partitioning: SpacialPartitioning<20>,
}

impl VerletPhysicsWorld {
    pub fn new() -> VerletPhysicsWorld {
        VerletPhysicsWorld {
            objects: BTreeMap::new(),
            beams: BTreeMap::new(),
            spatial_partitioning: SpacialPartitioning::<20>::new(FixedPoint::new(5.0)),
        }
    }

    pub fn add_or_set_object(&mut self, object : VerletObject, id : Id) {
        self.objects.insert(id, Entry{val: object, is_dirty: true});
    }

    pub fn remove_object(&mut self, id : Id) {
        self.objects.remove(&id);
    }

    pub fn add_beam(&mut self, beam : VerletBeam, id : Id) {
        self.beams.insert(id, Entry{val: beam, is_dirty: true});
    }

    pub fn add_or_set_beam(&mut self, beam : VerletBeam, id : Id) {
        self.beams.insert(id, Entry{val: beam, is_dirty: true});
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