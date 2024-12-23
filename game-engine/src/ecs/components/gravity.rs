use bevy_ecs::{component::Component, system::Query};
use macroquad::prelude::*;

use super::velocity::*;

#[derive(Component, Default)]
pub struct Gravity {

    gravity : f32,
    multiplier : f32,
}

impl Gravity {

    pub fn default() -> Gravity {

        Gravity { gravity : 10.0, multiplier : 0.003 }
    }
}

pub fn apply_gravity(mut query : Query<(&Gravity, &mut Velocity)>) {

    for (gravity, mut velocity) in &mut query{

        velocity.vec.y += gravity.gravity * gravity.multiplier;
    }
}