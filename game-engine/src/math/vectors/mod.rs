pub mod vec2;

#[derive(Copy, Clone)]
pub struct Vec2 {

    pub x : f32,
    pub y : f32,
}

#[derive(Copy,Clone)]
pub struct Vec2I32 {

    pub x : i32,
    pub y : i32,
}

// create aaaaaaa macro to implement vectors

pub trait Vector2<NumType> {
    // The struct of the vector

    fn new(x:NumType, y:NumType) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn dot(v1 : Self, v2 : Self) ->NumType;
    fn magnitude(&self) -> NumType;
    //fn add(&mut self, v : Vector2f64) -> Vector2f64;
    //fn subtract(&mut self, v : Vector2f64) -> Vector2f64;
    fn multiply(self, k :NumType) -> Self;
    fn multiply_assign(&mut self, k : NumType);
    fn set_xy(&mut self, x :NumType, y : NumType);
    fn set(&mut self, value :NumType);
}