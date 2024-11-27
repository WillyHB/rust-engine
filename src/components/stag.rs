use bevy_ecs::component::Component;
use macroquad::prelude::*;

#[derive(Component)]
pub struct STag{

    pub tag : String,
}