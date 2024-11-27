use bevy_ecs::{component::Component, query::{With, Without}, system::Query};
use macroquad::prelude::*;

use crate::Position;

use super::collider::{self, Collider};

#[derive(Component)]
pub struct Velocity {

    pub vec : Vec2,
}

pub fn apply_velocity(mut query : Query<(&mut Position, &Velocity)>,) {
    
    for (mut position, velocity) in &mut query {

        position.vec += velocity.vec;
    }
}