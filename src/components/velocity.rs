use std::rc::Rc;

use bevy_ecs::{component::Component, query::{With, Without}, system::Query};
use macroquad::prelude::*;

use crate::Position;

use super::{collider::{self, Collider}, component::{Entity, MyComponent, ECS}};

#[derive(Component)]
pub struct Velocity {

    pub vec : Vec2,
}

impl Velocity {

    pub fn new() -> Self {

        Velocity { vec : Vec2::new(6_f32, 9_f32) }
    }
}

impl MyComponent for Velocity {

    fn update(&mut self, entity : &Entity) {
        
        println!("Velocity is Updating!");
    }
}

pub fn apply_velocity(mut query : Query<(&mut Position, &Velocity)>,) {
    
    for (mut position, velocity) in &mut query {

        position.vec += velocity.vec;
    }
}