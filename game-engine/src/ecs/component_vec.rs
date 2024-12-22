use std::{any::Any, cell::RefCell};

use super::component::MyComponent;

pub trait ComponentVec {

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&mut self);
    fn insert_none(&mut self, index : usize);
    //fn insert(&mut self, index : usize, component : ComponentType);
}

pub type ComponentVecType<T> = RefCell<Vec<Option<Box<T>>>>;

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