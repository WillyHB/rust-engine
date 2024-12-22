pub trait MyComponent {

    //fn start()
    //fn late_update?
    //fn awake()
    //fn destroy - Or make this an event of some sort?
    fn update(&mut self, entity : &Entity);
    // OR HAVE ACTIONS, THAT ARE RETURNED AND THEN PERFORMED
    // Send in a list of all components?
}



#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity {

    id : usize,
}

impl Entity {

    pub fn id(&self) -> usize { self.id }
    pub fn new(id : usize) -> Entity { Entity { id } } 
}
