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

    /*
    pub fn get_query_result<'a, 'w, T: WorldQuery>(
        &self,
        id: Id,
        query: Query<'a, 'w, (Entity, T)>,
    ) -> Option<(Entity, &'a T::Fetch::Item)>
        where
            T::Fetch: bevy_ecs::query::Fetch<'a>,
    {
        if let Some(entity) = self.get(id) {
            match query.get(entity) {
                Ok((_, component)) => Some((entity, component)),
                Err(_) => None,
            }
        } else {
            None
        }
    }

     */

    //pub fn get_query_result<'w, 's, Q, F>(id : Id, query: Query<'w, 's, Q, F>) -> Result<ROQueryItem<'w, Q>, QueryEntityError>
    //    where
    //        Q: WorldQuery + 'w,
    //        F: WorldQuery + 'w,
    //{
    //}

    //pub fn get_query_result<'w, 's, Q, F>(query: &mut Query<'w, 's, Q, F>, entity: Entity) -> Option<<<<Q as WorldQuery>::ReadOnly as WorldQueryGats>::Fetch as Fetch<'_>>::Item>
    //    where
    //        Q: WorldQuery + 'w,
    //        F: WorldQuery + 'w,
    //{
    //    query.get(entity).ok()
    //}

    // TQuery is <'w, 's, Q: WorldQuery, F: WorldQuery> Query<'w, 's, Q, F>


    // like that but return Result<ROQueryItem<'_, Q>, QueryEntityError>
    //pub fn get_query_result<Q: WorldQuery>(&self, id: Id, query: &Query<Q>) -> Result<ROQueryItem<'_, Q>, QueryEntityError> {
    //    match self.map.get(&id) {
    //        Some(entity) => {
    //            query.get(*entity)
    //        },
    //        None => Err(QueryEntityError::NoSuchEntity(Entity::from_raw(0))),
    //    }
    //}

    // Q: how can I describe something like Query<(Entity, &TankBubble, &Position, &Rigidbody)> in a generic way?
    //    I want to be able to pass in a query into my get_query_result function
    //    and then return a Result<ROQueryItem<'_, Q>, QueryEntityError>
    // A: I think I need to use a macro to generate the function
    // Q: show me

    //pub fn get_query_result with the macro




    // now I can use it like this:
    //pub fn get_query_result<Q: WorldQuery>(&self, id: Id, query: &Query<Q>) -> Result<ROQueryItem<'_, Q>, QueryEntityError> {
    //    get_query_result!(self, id, query)
    //}

    // or like this:
    //pub fn get_query_result<Q: WorldQuery>(&self, id: Id, query: &Query<Q>) -> Result<ROQueryItem<'_, Q>, QueryEntityError> {
    //    get_query_result!(self, id, query)
    //}





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

