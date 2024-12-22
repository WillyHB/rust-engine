use core::fmt;
use std::{ops::Mul, process::Output};

use crate::vectors::*;


impl std::ops::Add for Vec2 {

    type Output = Self;
     
    fn add(self, other : Self) -> Self {

        Self { x : self.x + other.x, y : self.y + other.y }
    }
}

impl std::ops::Sub for Vec2 {

    type Output = Self;
     
    fn sub(self, other : Self) -> Self::Output {

        Self { x : self.x - other.x, y : self.y - other.y }
    }
}

impl std::ops::AddAssign for Vec2 {

    fn add_assign(&mut self, rhs: Self) {
        
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Vec2 {

    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Vector2<f32> for Vec2 {

    fn new(x:f32,y:f32) -> Self {

        Self {x,y}
    }

    fn zero() -> Self {

        Self {x:0.0,y:0.0}
    }

    fn one() -> Self {

        Self {x:1.0,y:1.0}
    }

    fn dot (v1 : Self, v2 : Self) -> f32{

        v1.x * v2.x + v1.y * v2.y
    }

    fn magnitude(&self) -> f32 {

        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    fn multiply(self, k : f32) -> Self {

        Self{
        x : self.x * k,
        y : self.y * k,
        }

    }

    fn multiply_assign(&mut self, k : f32) {

        self.x *= k;
        self.y *= k;
    }

    fn set_xy(&mut self, x : f32, y : f32) {

        self.x = x;
        self.y = y;
    }

    fn set(&mut self, value : f32) {

        self.x = value;
        self.y = value;
    }
}

impl fmt::Display for Vec2 {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "({}, {})", self.x, self.y)
    }
}