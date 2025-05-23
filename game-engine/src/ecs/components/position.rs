use bevy_ecs::{component::Component};

use crate::{ecs::component::{Entity, MyComponent}, math::vectors::{ Vec2f32}};


#[derive(Component)]
pub struct Position {

    pub vec : Vec2f32,
}

impl MyComponent for Position {

    fn update(&mut self, entity : &Entity) {
        
        println!("Ohhh Now position updating");
        //self.vec += entity.get_component::<Velocity>().expect("bruh").vec;
    }
}