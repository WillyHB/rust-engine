use bevy_ecs::{component::Component};

use crate::vectors::Vec2;

use super::component::{Entity, MyComponent};


#[derive(Component)]
pub struct Position {

    pub vec : Vec2,
}

impl MyComponent for Position {

    fn update(&mut self, entity : &Entity) {
        
        println!("Ohhh Now position updating");
        //self.vec += entity.get_component::<Velocity>().expect("bruh").vec;
    }
}