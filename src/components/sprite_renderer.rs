use crate::animated_sprite::AnimatedSprite;
use macroquad::{prelude::*, time};
use bevy_ecs::{component::Component, query::Without, system::Query};

use crate::Position;

use super::sprite::Sprite;

#[derive(Component)]
pub struct SpriteRenderer {
    render_target : Option<RenderTarget>,
}

impl SpriteRenderer {

    pub fn new(render_target : Option<RenderTarget>) -> SpriteRenderer {

        SpriteRenderer { render_target }
    }
}

pub fn render_sprites(mut query : Query<(&SpriteRenderer, &Sprite, &Position), Without<AnimatedSprite>>) {

    for (renderer, sprite, position) in &mut query {

        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
        cam.render_target = renderer.render_target.clone();
        set_camera(&cam);

        let mut params = DrawTextureParams::default();
        params.dest_size = Some(Vec2::new(sprite.destination_bounds.w, sprite.destination_bounds.h));
        params.rotation = sprite.rotation;
        params.source = sprite.source;
        params.flip_x = sprite.flip_x;
        params.flip_y = sprite.flip_y;

        draw_texture_ex(&sprite.texture, position.vec.x as f32, position.vec.y as f32, sprite.colour, params);
    }
}

pub fn render_animated_sprites(mut query : Query<(&SpriteRenderer, &mut AnimatedSprite, &Position), Without<Sprite>>) {

    for(renderer,anim_sprite,position) in &mut query {

        if !anim_sprite.animator.has_animation() { return; }

        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
        cam.render_target = renderer.render_target.clone();
        set_camera(&cam);

        let i : usize = anim_sprite.animator.current_frame() as usize;

        let sprite = match anim_sprite.animator.get_frame(i) {

            Ok(x) => {x},
            Err(_) => { continue; }
        };

        //let sprite = &anim_sprite.animator.get_frame(i);
        let mut params = DrawTextureParams::default();
        //params.dest_size = Some(Vec2::new(sprite.frames[i].destination_bounds.w, sprite.frames[i].destination_bounds.h));
        
        params.dest_size = Some(

            Vec2::new(50.0, 50.0)
        );

        params.rotation = sprite.rotation;
        params.source = sprite.source;
        params.flip_x = anim_sprite.flip_x ^ sprite.flip_x;
        params.flip_y = anim_sprite.flip_y ^ sprite.flip_y;

        draw_texture_ex(&sprite.texture, position.vec.x as f32, position.vec.y as f32, sprite.colour, params);
    }
}