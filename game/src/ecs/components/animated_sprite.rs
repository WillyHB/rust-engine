use bevy_ecs::{component::Component, system::Query};
use macroquad::prelude::*;

use crate::{animation::{Animation, AnimationParams}, animator::Animator};

use super::sprite::Sprite;

#[derive(Component)]
pub struct AnimatedSprite {
    pub animator : Animator<Sprite>,
    pub flip_x : bool,
    pub flip_y : bool,
}

impl AnimatedSprite {

    pub fn new(animator : Animator<Sprite>) -> AnimatedSprite {

        AnimatedSprite { animator, flip_x : false, flip_y : false }
    }


}

pub fn update_animation(mut query : Query<&mut AnimatedSprite>) {

    for mut sprite in &mut query {
        sprite.animator.update();
    }
}