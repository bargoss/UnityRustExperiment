use std::collections::HashMap;
use bevy_ecs::prelude::{Entity, Query, World};
use bevy_ecs::query::{Fetch, QueryEntityError, ROQueryItem, WorldQuery, WorldQueryGats};
use crate::game_core::view_components::Id;

pub struct IdEntityMap {
    map: HashMap<Id, Entity>,
}

impl IdEntityMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: Id, entity: Entity) {
        self.map.insert(id, entity);
    }

    pub fn get(&self, id: Id) -> Option<Entity> {
        self.map.get(&id).copied()
    }

    pub fn get_from_query<'w, Q: WorldQuery + WorldQuery<ReadOnly = Q>>(
        &self,
        query: &'w Query<'w, 'w, Q>,
        id: Id,
    ) -> Result<<<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item, QueryEntityError> {
        match self.get(id) {
            Some(entity) => {
                query.get(entity)
            }
            None => {
                Err(QueryEntityError::QueryDoesNotMatch(Entity::from_raw(0)))
            }
        }
    }





    pub fn remove(&mut self, id: Id) {
        self.map.remove(&id);
    }

    pub fn contains(&self, id: Id) -> bool {
        self.map.contains_key(&id)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

