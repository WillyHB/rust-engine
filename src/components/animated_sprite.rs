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


    pub async fn animation_from_spritesheet(tag : &str, sheet : Texture2D, size_x : u32, size_y : u32, row : u32, num : u8, fps : u32, params : &AnimationParams) -> Animation<Sprite> {

        let mut frames : Vec<Sprite> = Vec::new(); 

        for i in 0..num {

            let frame = sheet.clone();
            frame.set_filter(FilterMode::Nearest);
            let s = Sprite::new(
                Some(frame),
                Rect::new(0.0,0.0, size_x as f32, size_y as f32), 
                    WHITE, 
                    Some(Rect::new(i as f32 *size_x as f32, row as f32 * size_y as f32, size_x as f32, size_y as f32))).await;
            frames.push(s);
        };

        Animation::new(tag, frames, fps, &params)
    }
}

pub fn update_animation(mut query : Query<&mut AnimatedSprite>) {

    for mut sprite in &mut query {
        sprite.animator.update();
    }
}