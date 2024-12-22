use std::{any::{Any, TypeId}, borrow::{Borrow, BorrowMut}, cell::{RefCell, RefMut}, collections::{hash_map::IterMut, HashMap}, ops::{Deref, DerefMut}, ptr::null_mut, rc::Rc};

use bevy_ecs::system;
use macroquad::math::Vec2;

use crate::Velocity;

use super::query::{ComponentBundle, Query};

pub trait MyComponent {

    //fn start()
    //fn late_update?
    //fn awake()
    //fn destroy - Or make this an event of some sort?
    fn update(&mut self, entity : &Entity);
    // OR HAVE ACTIONS, THAT ARE RETURNED AND THEN PERFORMED
    // Send in a list of all components?
}

pub trait ComponentVec {

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&mut self);
    fn insert_none(&mut self, index : usize);
    //fn insert(&mut self, index : usize, component : ComponentType);
}

type ComponentVecType<T> = RefCell<Vec<Option<Box<T>>>>;

impl<T : MyComponent + 'static> ComponentVec for ComponentVecType<T> {

    fn as_any(&self) -> &dyn Any{

        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{

        self as &mut dyn Any
    }

    fn push_none(&mut self) {
        
        self.get_mut().push(None);
    }

    fn insert_none(&mut self, index : usize) {
        self.get_mut().insert(index, None);
    }

}


#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity {

    id : usize,
}

pub struct ECS {

    component_map : HashMap<TypeId, Box<dyn ComponentVec>>,
    resources : HashMap<String, Box<dyn Any>>,
    number_of_entities : usize,
    entity_id : usize,
    systems : Vec<Box<dyn Fn(Query)>>,
}

impl ECS {

    pub fn new() -> ECS {

        ECS { 
            component_map : HashMap::new(), 
            number_of_entities : 0, 
            entity_id : 0, 
            resources : HashMap::new(),
            systems : Vec::new(),
        }
    }

    
    pub fn add_system(&mut self, system : impl Fn(Query) + 'static) {

        self.systems.push(Box::new(system));

    }

    pub fn run_systems(&self) {

        for sys in &self.systems {

            sys(Query::new(ComponentBundle::new()));
            
        }
    }

    /// Adds the component to the ECS
    /// If component already registered, it does nothing
    pub fn register<T : MyComponent + 'static>(&mut self) {

        let id = TypeId::of::<T>();

        if self.component_map.contains_key(&id) { return };

        self.component_map.insert(id, Box::new(RefCell::new(Vec::<Option<Box<T>>>::new())));
        let vec = self.component_map.get_mut(&id).unwrap();

        for _ in 0..self.entity_id {

            vec.push_none();
        }
    }

    /// Calls register<T>() if component not already registered
    pub fn add_component<T : MyComponent + 'static>(&mut self, component : T, entity : &Entity) {
        
        // Gets id of the component to return
        let id = TypeId::of::<T>();

        // Calls register - function does nothing if component already registered
        self.register::<T>();
            
        // Get component vec and insert component at id index
        let vec = self.component_map.get_mut(&id).unwrap().as_any_mut().downcast_mut::<ComponentVecType<T>>().unwrap().get_mut();
        vec.insert(entity.id, Some(Box::new(component)));
    }

    
    /// Borrows a component from the ECS mutably
    /// 
    /// Will panic if same component is borrowed at the same time
    /// 
    /// Calls register<T>() if component not already registered
    pub fn borrow_components<T : MyComponent + 'static>(&mut self) -> &mut ComponentVecType<T> {

        self.register::<T>();

        self.component_map.get_mut(&TypeId::of::<T>()).unwrap().as_any_mut().downcast_mut::<ComponentVecType<T>>().unwrap()
    }

    pub fn instantiate_entity(&mut self) -> Entity {

        let id = self.entity_id;
        self.entity_id += 1;

        self.number_of_entities += 1;

        for component_vec in self.component_map.values_mut() {

            component_vec.push_none();
        }

        Entity { id }
    }

    pub fn kill_entity(&mut self, entity : &Entity) {

        self.number_of_entities -= 1;

        for c_vec in self.component_map.values_mut() {

            c_vec.insert_none(entity.id);
        }

    }


    // ACCESS LIKE EVENTS OF SOME SORT??
    // LIKE EVENTCHANNELS IN UNITY

}

