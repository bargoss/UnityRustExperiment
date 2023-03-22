use std::collections::HashMap;
use bevy_ecs::prelude::*;
use bevy_ecs::query::{QueryEntityError, ROQueryItem, WorldQuery};
use crate::game_core::view_components::Id;

#[derive(Resource)]
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

    /*
    #[inline]
    pub fn get(&self, entity: Entity) -> Result<ROQueryItem<'_, Q>, QueryEntityError> {
        // SAFETY: system runs without conflicts with other systems.
        // same-system queries have runtime borrow checks when they conflict
        unsafe {
            self.state.as_readonly().get_unchecked_manual(
                self.world,
                entity,
                self.last_change_tick,
                self.change_tick,
            )
        }
    }
    */


    //fn get_component_from_query_wrapper<'a, Q: WorldQuery>(
    //    query: &Query<Q>,
    //    entity: Entity,
    //) -> Result<ROQueryItem<'a, Q>, QueryEntityError> {
    //    query.get(entity)
    //}
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
    /*
    pub fn get_from_query<Q: WorldQuery>(
        query: &Query<Q>,
        id: Id,
    ) -> Result<ROQueryItem<'_, Q>, QueryEntityError> {
        match self.get(id) {
            Some(entity) => {
                Self::get_component_from_query_wrapper(query, entity);
            }
            None => {
                Err(QueryEntityError::QueryDoesNotMatch(Entity::from_raw(0)))
            }
        }
    }
     */


    /*
    pub fn get_from_query2<'w, Q: WorldQuery + WorldQuery<ReadOnly = Q>>(
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

     */





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

