use std::collections::HashMap;
use bevy_ecs::prelude::{Entity, Query, World};
use bevy_ecs::query::{QueryEntityError, ROQueryItem, WorldQuery};
use crate::game_core::view_components::Id;

macro_rules! get_query_result {
        ($self:ident, $id:ident, $query:ident) => {
            match $self.map.get(&$id) {
                Some(entity) => {
                    $query.get(*$entity)
                },
                None => Err(QueryEntityError::NoSuchEntity(Entity::from_raw(0))),
            }
        };
    }

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

