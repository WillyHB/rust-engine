use bevy_ecs::component::Component;
use macroquad::prelude::*;

#[derive(Component)]
pub struct ITag{

    pub tag : u8,
}