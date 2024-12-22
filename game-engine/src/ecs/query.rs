use std::any::TypeId;

use super::component::MyComponent;


// CREATE MACRO HERE
pub struct Query/*<T1,T2,...>*/ {
    bundle : ComponentBundle,

}

#[macro_export]
macro_rules! query {
    () => {
        
    };
}

impl Query {

    pub fn new(bundle : ComponentBundle) -> Query {

        Query { bundle }
    }
}

pub trait ComponentBundleTrailt {

}

pub struct ComponentBundle {

    components : Vec<TypeId>,
}

impl ComponentBundle {

    pub fn new() -> ComponentBundle {

        ComponentBundle { components : Vec::new() }
    }

    pub fn push<T : MyComponent + 'static>(&mut self) {
        self.components.push(TypeId::of::<T>());
    }
}