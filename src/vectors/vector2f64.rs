use core::fmt;
use std::{ops::Mul, process::Output};

use crate::vectors::*;

impl std::ops::Add for Vector2f64 {

    type Output = Self;
     
    fn add(self, other : Self) -> Self {

        Self { x : self.x + other.x, y : self.y + other.y }
    }
}

impl std::ops::Sub for Vector2f64 {

    type Output = Self;
     
    fn sub(self, other : Self) -> Self::Output {

        Self { x : self.x - other.x, y : self.y - other.y }
    }
}

impl std::ops::AddAssign for Vector2f64 {

    fn add_assign(&mut self, rhs: Self) {
        
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Vector2f64 {

    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Vector2 for Vector2f64 {

    fn new(x:f64,y:f64) -> Vector2f64 {

        Vector2f64 {x,y}
    }

    fn zero() -> Vector2f64 {

        Vector2f64 {x:0.0,y:0.0}
    }

    fn one() -> Vector2f64 {

        Vector2f64 {x:1.0,y:1.0}
    }

    fn dot (v1 : Vector2f64, v2 : Vector2f64) -> f64 {

        v1.x * v2.x + v1.y * v2.y
    }

    fn magnitude(&self) -> f64 {

        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    fn multiply(self, k : f64) -> Vector2f64 {

        Vector2f64{
        x : self.x * k,
        y : self.y * k,
        }

    }

    fn multiply_assign(&mut self, k : f64) {

        self.x *= k;
        self.y *= k;
    }

    fn set_xy(&mut self, x : f64, y : f64) {

        self.x = x;
        self.y = y;
    }

    fn set(&mut self, value : f64) {

        self.x = value;
        self.y = value;
    }
}

impl fmt::Display for Vector2f64 {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "({}, {})", self.x, self.y)
    }
}