use bevy_ecs::{component::Component, query::{With, Without}, system::Query};
use macroquad::prelude::*;

use crate::{animation::{Animation, AnimationParams}, input, statemachine::{State, Statemachine}, time::Timer, Position, Speed, Velocity};

use super::{animated_sprite::AnimatedSprite, collider::Collider, sprite::Sprite};

#[derive(Component)]
pub struct Player {

    pub run_anim : Animation<Sprite>,
    pub walk_anim : Animation<Sprite>,
    pub idle_anim : Animation<Sprite>,
    pub fall_anim : Animation<Sprite>,
    pub jump_anim : Animation<Sprite>,
    pub land_anim : Animation<Sprite>,

    pub idle_state : IdleState,
    pub run_state : RunState,
    pub fall_state : FallState,
    pub walk_state : WalkState,
    pub land_state : LandState,
    pub jump_state : JumpState,

    is_jumping : bool,
    pub statemachine : Statemachine,
}

impl Player {

    pub async fn new() -> Player {

        let mut params = AnimationParams {
            reverse : false,
            repeat : true,
            start_frame : 0,
        };

        let player_texture = load_texture("assets/player_sheet.png").await.unwrap();
        let walk_anim = AnimatedSprite::animation_from_spritesheet("walk", player_texture.clone(), 16, 20, 5, 8, 12, &params).await;
        let idle_anim = AnimatedSprite::animation_from_spritesheet("idle", player_texture.clone(), 16, 20, 1, 4, 6, &params).await;
        let run_anim = AnimatedSprite::animation_from_spritesheet("run", player_texture.clone(), 16, 20, 4, 8, 12, &params).await;

        params.repeat = false;
        let fall_anim = AnimatedSprite::animation_from_spritesheet("fall", player_texture.clone(), 16, 20, 6, 3, 6, &params).await;
        let land_anim = AnimatedSprite::animation_from_spritesheet("land", player_texture.clone(), 16, 20, 3, 6, 12, &params).await;
        let jump_anim = AnimatedSprite::animation_from_spritesheet("jump", player_texture.clone(), 16, 20, 2, 4, 12, &params).await;

        params.repeat = false;

        Player {

            run_anim : run_anim,
            walk_anim : walk_anim,
            idle_anim : idle_anim,
            fall_anim : fall_anim,
            jump_anim : jump_anim,
            land_anim : land_anim,
            idle_state : IdleState {},
            run_state : RunState {},
            fall_state : FallState {},
            walk_state : WalkState {},
            land_state : LandState {},
            jump_state : JumpState {},
            is_jumping : false,
            statemachine : Statemachine::new(Box::new(IdleState{})),
        }
    }

    pub fn is_grounded(&self, player_collider : &Collider, collider_query : &Query<&Collider, Without<Player>>) -> bool {

        for col in &mut collider_query.into_iter() {

            if String::from("ground") == col.tag && player_collider.collides(col.tag.clone()) {

                return true;
            }
        }

        return false;
    }
}

pub fn player_update(mut query : Query<(&mut Velocity, &Speed, &Collider, &mut AnimatedSprite, &mut Player)>,
collider_query : Query<&Collider, Without<Player>>) {

    for (mut velocity, speed, collider, mut sprite, mut player) in &mut query {

        if input::is_pressed(KeyCode::D) {

            player.statemachine.transition(Box::new(IdleState {}));
            sprite.flip_x = false;
            velocity.vec.x = speed.speed;
        }

        else if input::is_pressed(KeyCode::A) {

            player.statemachine.transition(Box::new(RunState {}));
            sprite.flip_x = true;
            velocity.vec.x = -speed.speed;
        }

        if !player.is_grounded(collider, &collider_query) {

            if !player.is_jumping {
                sprite.animator.play_once(Animation::from(&player.fall_anim));
            }
            continue; 
        } 

        player.statemachine.update();

        player.is_jumping = false;

        if input::is_pressed(KeyCode::W) {velocity.vec.y = -1.5; player.is_jumping = true }

        if input::is_pressed(KeyCode::D) { 

            sprite.animator.play(Animation::from(&player.run_anim));
        }
        else if input::is_pressed(KeyCode::A) {

            sprite.animator.play(Animation::from(&player.run_anim));
        }
        else { 

            sprite.animator.play(Animation::from(&player.idle_anim));
            if velocity.vec.x.abs() > 0.0 {
                velocity.vec.x -= if velocity.vec.x.abs() < 0.0001 {
                    velocity.vec.x
                }
                else {
                0.1 * velocity.vec.x.signum()
                }
            }
        }

    }
}

struct IdleState;
struct RunState;
struct WalkState;
struct JumpState;
struct FallState;
struct LandState;

impl State for WalkState {

    fn get_tag(&self) -> String {
        
        String::from("walk")
    }

    fn update(&self) {
        
    }

    fn enter(&self) {
        
    }

    fn exit(&self) {
        
    }
}
impl State for JumpState {

    fn get_tag(&self) -> String {
        
        String::from("jump")
    }

    fn update(&self) {
        
    }

    fn enter(&self) {
        
    }

    fn exit(&self) {
        
    }
}
impl State for FallState {

    fn get_tag(&self) -> String {
        
        String::from("fall")
    }

    fn update(&self) {
        
    }

    fn enter(&self) {
        
    }

    fn exit(&self) {
        
    }
}
impl State for LandState {

    fn get_tag(&self) -> String {
        
        String::from("land")
    }

    fn update(&self) {
        
    }

    fn enter(&self) {
        
    }

    fn exit(&self) {
        
    }
}
impl State for RunState {

    fn get_tag(&self) -> String {
        
        String::from("run")
    }

    fn update(&self) {
        
        println!("is running?");
    }

    fn enter(&self) {
        
    }

    fn exit(&self) {
        
    }
}
impl State for IdleState {

    fn get_tag(&self) -> String {
        
        String::from("idle")
    }

    fn update(&self) {
        
        println!("is idle :)");
    }

    fn enter(&self) {
        
    }

    fn exit(&self) {
        
    }
}