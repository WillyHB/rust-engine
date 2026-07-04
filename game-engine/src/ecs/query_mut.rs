use std::{any::TypeId, collections::HashMap};

use super::{component::MyComponent, component_vec::ComponentVec};

pub struct QueryMut<'a> {
    pub component_map: &'a mut HashMap<TypeId, Box<dyn ComponentVec>>,
}

impl<'a> QueryMut<'a> {

    pub fn new(map : &mut HashMap<TypeId, Box<dyn ComponentVec>>) -> QueryMut {

        QueryMut { component_map : map }
    }


    pub fn get_with_component<T : MyComponent + 'static>(self) -> QueryMut<'a> {

        let map = self.component_map;

        QueryMut { component_map : map }
    }


    pub fn finalise(&mut self)  {

    }
}
