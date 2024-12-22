use animated_sprite::{AnimatedSprite};
use animator::Animator;
use collider::{apply_collision, Collider};
use ecs::component::ECS;
use macroquad::audio::{self, load_sound, PlaySoundParams};
use macroquad::camera::{self, set_camera, set_default_camera, Camera2D};
use macroquad::color::{self, rgb_to_hsl, Color, BLACK, BLUE, ORANGE, RED, WHITE};
use macroquad::input::KeyCode;
use macroquad::math::{Rect, };
use macroquad::texture::{draw_texture, draw_texture_ex, load_texture, render_target, set_default_filter_mode, DrawTextureParams, RenderTarget, Texture2D};
use macroquad::window::{clear_background, next_frame, request_new_screen_size, screen_height, screen_width, Conf};
use schedule::{IntoSystemConfigs, Schedule};
use bevy_ecs::*;
use bevy_ecs::world::World;

mod statemachine;

const WINDOW_SIZE : (i32, i32) = (800,600);
const WORLD_BOUNDS : (f32, f32) = (400.0, 300.0);
const VIRTUAL_RES : (u32, u32) = (540, 360);

fn window_conf() -> Conf {

    Conf {
        window_title : "Test".to_owned(),

        window_width : WINDOW_SIZE.0,
        window_height : WINDOW_SIZE.1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), String> {

    // MAKE FUNCTIONS CUSTOM
    // AND ALLOW THEM TO RECIEVE INPUT
    // LIKE STATEMACHINE?? SHARED VALUE?? CURRENT STATE TAG PERHAPS?? OR NOT MAYBE CAUSE THATLL BE DEFINED OUTSIDE WELL SEE

    let render_target : RenderTarget = render_target(VIRTUAL_RES.0, VIRTUAL_RES.1);
    render_target.texture.set_filter(macroquad::texture::FilterMode::Nearest);
    
    let mut world = World::default();
    let mut update_schedule = Schedule::default();
    let mut draw_schedule = Schedule::default();

    let mut ecs = ECS::new();
    let entity = ecs.instantiate_entity();
    ecs.add_component(Velocity {vec : Vec2::new(5.0, 0.0)}, &entity);
    {
    let velocities = ecs.borrow_components::<Velocity>();
    velocities.get_mut().get_mut(0).unwrap().as_mut().unwrap().vec.x = 3.0;
    }
    


    fn test(q : ecs::query::Query) {
        println!("System runs");

    }

    ecs.add_system(test);

    let velocities2 = ecs.borrow_components::<Velocity>();

    println!("PP{}", velocities2.get_mut().get_mut(0).unwrap().as_mut().unwrap().vec.x);

    for vel in velocities2.get_mut() {

        if let Some(val) = vel {

            val.vec.x += 0.01;
        }

    }



    update_schedule.add_systems((
        gravity::apply_gravity,
        player::player_update.after(apply_gravity),
        collider::apply_collision.after(apply_gravity),
        velocity::apply_velocity.after(apply_collision), 
    ));

    draw_schedule.add_systems((
        animated_sprite::update_animation,
        sprite_renderer::render_sprites,
        sprite_renderer::render_animated_sprites.after(animated_sprite::update_animation),
    ));

    let speed : f32 = 600.0 * 0.005;

    let _player = world.spawn((
        Position { vec : Vec2::new(10.0, 10.0)}, 
        Velocity { vec : Vec2::zero(), },
        AnimatedSprite::new(Animator::new()),
        SpriteRenderer::new(Some(render_target.clone())),
        Player::new().await,
        Speed { speed },
        Gravity::default(),
        Collider::new(Rect::new(0.0,0.0,50.0,50.0), String::from("player")),
    ) ).id();

    let _wall = world.spawn((
        Position { vec : Vec2::new(200.0, 0.0) },
        Collider::new(Rect::new(0.0,0.0,50.0,1000.0), String::from("walls")),
        Sprite::new(None, Rect::new(0.0, 0.0, 50.0, 1000.0), BLUE, None).await,
        SpriteRenderer::new(Some(render_target.clone())),

    )).id();

    let _ground = world.spawn(( 
        Position { vec : Vec2::new(0.0, 300.0) },
        Collider::new(Rect::new(0.0,0.0,1000.0,50.0), String::from("ground")),
    )).id();

    let _left_wall = world.spawn(( 
        Position { vec : Vec2::new(-50.0, 0.0) },
        Collider::new(Rect::new(0.0,0.0,50.0,1000.0), String::default()),
    )).id();

    let _obstacle = world.spawn((

        Position { vec : Vec2::new(150.0, 270.0) },
        Sprite::new(None, Rect::new(0.0,0.0,50.0,30.0), RED, None).await,
        SpriteRenderer::new(Some(render_target.clone())),
        Collider::new(Rect::new(0.0,0.0,50.0,30.0), String::from("ground")),
    )).id();
       
    let mut timer : Timer = Timer::new();

    'running : loop {

        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        cam.render_target = Some(render_target.clone());
        set_camera(&cam);

        clear_background(color::BLACK);

        //ecs.update();
        ecs.run_systems();
        update_schedule.run(&mut world);
        draw_schedule.run(&mut world);

        if input::is_pressed(KeyCode::Q) { break 'running; }

        timer.update();

        set_default_camera();
        let params = DrawTextureParams {

            dest_size : Some(macroquad::math::Vec2::new(screen_width(), screen_height())),
            flip_y : true,
            ..Default::default()
        };
        draw_texture_ex(&render_target.texture, 0.0, 0.0, WHITE, params);

        next_frame().await
    }
    Ok(())
}