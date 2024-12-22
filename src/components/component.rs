use std::{any::{Any, TypeId}, borrow::BorrowMut, cell::{RefCell, RefMut}, collections::{hash_map::IterMut, HashMap}, ops::{Deref, DerefMut}, ptr::null_mut, rc::Rc};

use macroquad::math::Vec2;

use crate::Velocity;

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

type ComponentVecType<T : MyComponent + 'static> = RefCell<Vec<Option<Box<T>>>>;

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
}

impl ECS {

    pub fn new() -> ECS {

        ECS { component_map : HashMap::new(), number_of_entities : 0, entity_id : 0, resources : HashMap::new()}
    }

    pub fn add_component<T : MyComponent + 'static>(&mut self, component : T, entity : &Entity) {
        
        // Gets id of the component to return
        let id = TypeId::of::<T>();

        // If component not registered
        if !self.component_map.contains_key(&id){

            self.component_map.insert(id, Box::new(RefCell::new(Vec::<Option<Box<T>>>::new())));
            let vec = self.component_map.get_mut(&id).unwrap();

            // For each entity, push None in the new component vec
            for _ in 0..self.entity_id {
                vec.push_none();
            }
        }
            
        // Get component vec and insert component at id index
        let vec = self.component_map.get_mut(&id).unwrap().as_any_mut().downcast_mut::<ComponentVecType<T>>().unwrap().get_mut();
        vec.insert(entity.id, Some(Box::new(component)));
    }

    /*
    pub fn borrow_components_mut<T : MyComponent + 'static>(&mut self) -> Option<RefCell<Vec<Option<Box<T>>>>> {

        if let Some(component_vec) = self.component_map.get_mut(&TypeId::of::<T>()) {


        }

    }*/

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

