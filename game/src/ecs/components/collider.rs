use std::collections::{HashMap, HashSet};

use bevy_ecs::{component::Component, entity::Entity, query::Without, system::Query};
use macroquad::prelude::*;

use crate::{Position, Velocity};

#[derive(Component)]
pub struct Collider {

    pub local_bounds : Rect,
    collisions : Vec<String>,
    pub tag : String,
}

impl Collider {

    pub fn new(local_bounds : Rect, tag : String) -> Collider {

        Collider {local_bounds, collisions : Vec::new(), tag }
    }

    pub fn has_tag(&self) -> bool {

        self.tag != String::default()
    }

    pub fn global_bounds(&self, pos : Vec2) -> Rect {

        Rect::new(self.local_bounds.x + pos.x, 
            self.local_bounds.y + pos.y, 
            self.local_bounds.w, 
            self.local_bounds.h)
    }

    pub fn collides(&self, tag : String) -> bool {

        self.collisions.iter().position(|r| *r == tag).is_some()
    }
}

pub fn apply_collision(mut actor_query : Query<(&Position, &mut Velocity, &mut Collider,)>,
                       mut solids_query : Query<(&Position, &mut Collider), Without<Velocity>>) {

    for (_, _, mut collider) in &mut actor_query  {

        collider.collisions.clear();
    }

    for (_, mut collider) in &mut solids_query  {

        collider.collisions.clear();

    }

    for (actor_pos, mut actor_velocity, mut actor_collider) in &mut actor_query {

        for (solid_position, solid_collider) in &mut solids_query {

            let predicted_rect_x = actor_collider.global_bounds(
                Vec2::new(actor_pos.vec.x + actor_velocity.vec.x, actor_pos.vec.y));
            let predicted_rect_y = actor_collider.global_bounds(
                Vec2::new(actor_pos.vec.x, actor_pos.vec.y + actor_velocity.vec.y));
                
            let solid_rect = solid_collider.global_bounds(
                macroquad::math::vec2(solid_position.vec.x, solid_position.vec.y));

            if predicted_rect_x.intersect(solid_rect).is_some() {

                actor_velocity.vec.x = 0.0;
                actor_collider.collisions.push(solid_collider.tag.clone());
            }

            if predicted_rect_y.intersect(solid_rect).is_some() {

                actor_velocity.vec.y = 0.0;
                actor_collider.collisions.push(solid_collider.tag.clone());
            }
        }

    }
}
