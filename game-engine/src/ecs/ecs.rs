use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell},
    collections::HashMap,
};

use super::{
    component::{Entity, MyComponent}, component_vec::{ComponentVec, ComponentVecType}, components::{position::Position, velocity::Velocity}, query::Query, query_mut::QueryMut
};

pub struct ECS {
    component_map: HashMap<TypeId, Box<dyn ComponentVec>>,
    resources: HashMap<String, Box<dyn Any>>,
    number_of_entities: usize,
    entity_id: usize,
    systems: Vec<Box<fn(Query)>>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            component_map: HashMap::new(),
            number_of_entities: 0,
            entity_id: 0,
            resources: HashMap::new(),
            systems: Vec::new(),
        }
    }

    pub fn query(&self) -> Query {

        Query::new(&self.component_map)
    }

    pub fn query_mut(&mut self) -> QueryMut {

        QueryMut::new(&mut self.component_map)
    }

    pub fn add_system(&mut self, system: fn(Query)) {
        self.systems.push(Box::new(system));
    }

    pub fn run_systems(&mut self) {

        let q = Query::new(&self.component_map);
        let q_mut = QueryMut::new(&mut self.component_map);

        let velocities = q_mut.get_with_component::<Velocity>(); // Hello?
        self.component_map = HashMap::new();

        for sys in &self.systems {
            //sys();
        }
    }

    /// Adds the component to the ECS
    /// If component already registered, it does nothing
    pub fn register<T: MyComponent + 'static>(&mut self) {
        let id = TypeId::of::<T>();

        if self.component_map.contains_key(&id) {
            return;
        };

        self.component_map
            .insert(id, Box::new(RefCell::new(Vec::<Option<Box<T>>>::new())));
        let vec = self.component_map.get_mut(&id).unwrap();

        for _ in 0..self.entity_id {
            vec.push_none();
        }
    }

    /// Calls register<T>() if component not already registered
    pub fn add_component<T: MyComponent + 'static>(&mut self, component: T, entity: &Entity) {
        // Gets id of the component to return
        let id = TypeId::of::<T>();

        // Calls register - function does nothing if component already registered
        self.register::<T>();

        // Get component vec and insert component at id index
        let vec = self
            .component_map
            .get_mut(&id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentVecType<T>>()
            .unwrap()
            .get_mut();
        vec.insert(entity.id(), Some(Box::new(component)));
    }

    /// Borrows a component from the ECS mutably
    ///
    /// Will panic if same component is borrowed at the same time
    ///
    /// Calls register<T>() if component not already registered
    pub fn borrow_components<T: MyComponent + 'static>(&mut self) -> &mut ComponentVecType<T> {
        self.register::<T>();

        self.component_map
            .get_mut(&TypeId::of::<T>())
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentVecType<T>>()
            .unwrap()
    }

    pub fn has_component<T : MyComponent + 'static>(&mut self, entity : &Entity) -> bool {
        self.register::<T>();

        self.component_map
            .get_mut(&TypeId::of::<T>())
            .unwrap().has(entity.id())
    }

    pub fn instantiate_entity(&mut self) -> Entity {
        let id = self.entity_id;
        self.entity_id += 1;

        self.number_of_entities += 1;

        for component_vec in self.component_map.values_mut() {
            component_vec.push_none();
        }

        Entity::new(id)
    }

    pub fn kill_entity(&mut self, entity: &Entity) {
        self.number_of_entities -= 1;

        for c_vec in self.component_map.values_mut() {
            c_vec.insert_none(entity.id());
        }
    }

    // ACCESS LIKE EVENTS OF SOME SORT??
    // LIKE EVENTCHANNELS IN UNITY
}
