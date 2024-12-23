use bevy_ecs::{component::Component, system::Query};
use macroquad::{camera::{set_camera, Camera2D}, color::{Color, WHITE}, math::{Rect, Vec2}, texture::{draw_texture_ex, load_texture, DrawTextureParams, RenderTarget, Texture2D}, window::clear_background};

#[derive(Clone, Component)] 
pub struct Sprite {

    pub texture : Texture2D,
    pub destination_bounds : Rect,
    pub colour : Color,
    pub rotation : f32,
    pub source : Option<Rect>,
    pub flip_x : bool,
    pub flip_y : bool,
}

impl Sprite {

    pub async fn new (
        texture : Option<Texture2D>,
        destination_bounds : Rect,
        colour : Color,
        source : Option<Rect>) -> Sprite {

        let texture : Texture2D = 
        match texture {
            Some(tex) => { tex }
            None => { load_texture("game-engine/default-assets/default.png").await.expect("No default image asset at path") }
        };

        Sprite { texture, destination_bounds, colour, rotation : 0.0, source, flip_x : false, flip_y : false }
    }
}
