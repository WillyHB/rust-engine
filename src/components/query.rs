use std::any::TypeId;

use super::component::MyComponent;


pub struct Query {
    bundle : ComponentBundle,

}

impl Query {

    pub fn new(bundle : ComponentBundle) -> Query {

        Query { bundle }
    }
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