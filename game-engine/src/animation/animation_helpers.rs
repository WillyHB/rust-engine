pub mod animation {
    use macroquad::{color::WHITE, math::Rect, texture::{FilterMode, Texture2D}};

    use crate::{animation::animation::{Animation, AnimationParams}, ecs::components::sprite::Sprite};


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