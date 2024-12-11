use std::{cell::RefCell, collections::{hash_map::IterMut, HashMap}, ops::{Deref, DerefMut}, rc::Rc};



pub trait MyComponent {

    //fn start()
    //fn late_update?
    //fn awake()
    //fn destroy - Or make this an event of some sort?
    fn update(&mut self, entity : &Entity, entities : &mut HashMap<Entity, Vec<Box<dyn MyComponent>>>);
    // OR HAVE ACTIONS, THAT ARE RETURNED AND THEN PERFORMED
    // Send in a list of all components?
}

pub trait ComponentVec {

    fn push_none(&mut self);
    fn remove(&mut self, entity : Entity) -> Result<Entity, &str>;
    fn as_any(&self) -> &dyn std::any::Any; 
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T : MyComponent + 'static> ComponentVec for RefCell<Vec<Option<T>>> {

    fn push_none(&mut self) {
        
        self.get_mut().push(None);
    }

    fn remove(&mut self, entity : Entity) -> Result<Entity, &str> {

        if entity.id > self.get_mut().len() || entity.id < 0 {

            return Err("Cannot remove entity, id is out of bounds (Is not instantiated in world)");
        }

        self.get_mut().remove(entity.id as usize);
        return Ok(entity);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    

}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity {

    id : usize,
}

pub struct ECS {

    component_vecs : Vec<Box<dyn ComponentVec>>,
    number_of_entities : usize,
}

impl ECS {

    pub fn new() -> ECS {

        ECS { component_vecs : Vec::new(), number_of_entities : 0}
    }

    pub fn update(&mut self) {

    }
    
    pub fn instantiate_entity(&mut self) -> Entity {
        let entity = Entity { id : self.number_of_entities };

        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }

        self.number_of_entities+=1;
        entity
    }

    
    pub fn destroy_entity(&mut self, entity : Entity) {
        
        for component_vec in self.component_vecs.iter_mut() {

            component_vec.remove(entity);
        }
    }
    

    pub fn add_component<T : MyComponent + 'static>(&mut self, component : T, entity : &Entity) 
    -> Result<Box<dyn ComponentVec + 'static>, Box<dyn ComponentVec + 'static>> {
        
        for component_vec in self.component_vecs.iter_mut() {

            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut()::<Vec<Option<T>>() {

                //component_vec.insert(Some(component), entity.id);
                Ok(component)
            }
        }

        Err(component);
    }

    pub fn borrow_component_mut<T : MyComponent + 'static>(&mut self, entity : &Entity) 
    -> Option<&mut Box<dyn MyComponent + 'static>> {

        let x = self.entities.get_mut(entity);
        
        if let Some(a) = x {
            a.into_iter().find(|x| x.as_any().downcast_ref::<T>().is_some())
        } else { None }
    }

    pub fn borrow_component<T : MyComponent + 'static>(&self, entity : &Entity) -> Option<&Box<dyn MyComponent + 'static>> {

    }
}

