pub mod vector2f64;

#[derive(Copy, Clone)]
pub struct Vector2f64 {

    pub x : f64,
    pub y : f64,
}

pub trait Vector2 {
    // The struct of the vector

    fn new(x:f64, y:f64) -> Vector2f64;
    fn zero() -> Vector2f64;
    fn one() -> Vector2f64;
    fn dot(v1 : Vector2f64, v2 : Vector2f64) ->f64;
    fn magnitude(&self) -> f64;
    //fn add(&mut self, v : Vector2f64) -> Vector2f64;
    //fn subtract(&mut self, v : Vector2f64) -> Vector2f64;
    fn multiply(self, k :f64) -> Vector2f64;
    fn multiply_assign(&mut self, k : f64);
    fn set_xy(&mut self, x :f64, y : f64);
    fn set(&mut self, value :f64);
}