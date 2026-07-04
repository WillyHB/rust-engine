use std::{any::{Any, TypeId}, cell::RefCell, collections::HashMap, rc::Rc};

use super::{component::MyComponent, component_vec::{ComponentVec, ComponentVecType}};

// CREATE MACRO HERE
pub struct Query<'a> {
    component_map: &'a HashMap<TypeId, Box<dyn ComponentVec>>,
    to_find : Vec<TypeId>,
    to_remove : Vec<TypeId>,

    
}

impl<'a> Query<'a> {

    pub fn new(map : &HashMap<TypeId, Box<dyn ComponentVec>>) -> Query {

        Query { component_map : map, to_find : Vec::new(), to_remove : Vec::new() }
    }

    pub fn with_component<T : MyComponent + 'static>(self) -> Query<'a> {

        let map = self.component_map;
        let mut to_find = self.to_find;
        let to_remove = self.to_remove;

        to_find.push(TypeId::of::<T>());
        
        Query { component_map : map, to_find, to_remove }
    }
    
    pub fn without_component<T : MyComponent + 'static>(self) -> Query<'a> {

        let map = self.component_map;
        let to_find = self.to_find;
        let mut to_remove = self.to_remove;

        to_remove.push(TypeId::of::<T>());

        Query { component_map : map, to_find, to_remove }
    }

    pub fn finalise(&self) -> Vec<(usize, Box<dyn ComponentVec>)> {

        //Be able to loop over as a tuple of components
        let filter = self.component_map.iter().filter(|x| self.to_find.contains(x.0) && !self.to_remove.contains(x.0));

        let entity_vec : Vec<(usize, Box<dyn ComponentVec>)> = vec![];
        for (type_id, component_vec) in filter.into_iter() {

            entity_vec.push((0, Com));
        }
        let iter : Vec<(&TypeId, &Box<dyn ComponentVec>)> = filter.collect();

        vec
    }
}
