use std::rc::Rc;

use bevy_ecs::{component::Component, query::{With, Without}, system::Query};
use macroquad::prelude::*;

use crate::Position;

use super::{collider::{self, Collider}, component::{Entity, MyComponent, ECS}};

#[derive(Component)]
pub struct Velocity {

    pub vec : Vec2,
}

/*
impl MyComponent for Velocity {

    fn update(&mut self, entity : &Entity, entities : Rc<&mut ECS>) {
        
        println!("Velocity is Updating!");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
    */

pub fn apply_velocity(mut query : Query<(&mut Position, &Velocity)>,) {
    
    for (mut position, velocity) in &mut query {

        position.vec += velocity.vec;
    }
}