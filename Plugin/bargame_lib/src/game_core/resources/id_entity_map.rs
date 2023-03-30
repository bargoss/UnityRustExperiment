use std::collections::HashMap;
use bevy_ecs::prelude::*;
use bevy_ecs::query::{QueryEntityError, QueryItem, ROQueryItem, WorldQuery};
use crate::game_core::common::*;

#[derive(Default, Resource)]
pub struct IdEntityMap {
    map: HashMap<Id, Entity>,
}

impl IdEntityMap {
    pub fn insert(&mut self, id: Id, entity: Entity) {
        self.map.insert(id, entity);
    }

    pub fn get(&self, id: Id) -> Option<Entity> {
        self.map.get(&id).copied()
    }

    fn get_component_from_query_wrapper<'a, Q: WorldQuery>(
        query: &'a Query<'a, '_, Q>,
        entity: Entity,
    ) -> Result<ROQueryItem<'a, Q>, QueryEntityError> {
        query.get(entity)
    }
    pub fn get_from_query<'a, Q: WorldQuery>(
        &self,
        query: &'a Query<'a, '_, Q>,
        id: Id,
    ) -> Option<ROQueryItem<'a, Q>> {
        match self.get(id) {
            Some(entity) => {
                match Self::get_component_from_query_wrapper(query, entity) {
                    Ok(component) => Some(component),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
    fn get_mut_component_from_query_wrapper<'a, Q: WorldQuery>(
        query: &'a mut Query<'a, '_, Q>,
        entity: Entity,
    ) -> Result<QueryItem<'a, Q>, QueryEntityError> {
        query.get_mut(entity)
    }

    pub fn get_mut_from_query<'a, 'b, Q: WorldQuery>(
        &self,
        query: &'a mut Query<'b, '_, Q>,
        id: Id,
    ) -> Option<QueryItem<'a, Q>>
        where
            'b: 'a,
    {
        match self.get(id) {
            Some(entity) => {
                match query.get_mut(entity) {
                    Ok(component) => Some(component),
                    Err(_) => None,
                }
            }
            None => None,
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

