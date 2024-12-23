pub mod vec2;
mod implement;


pub use implement::impl_vector2;

use std::fmt;



impl_vector2!(Vec2f32, f32);
impl_vector2!(Vec2f64, f64);
impl_vector2!(Vec2i32, i32);


/// A trait to outline the structure of any 2D vector
pub trait Vector2<NumType> {
    // The struct of the vector

    fn new(x:NumType, y:NumType) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn dot(v1 : Self, v2 : Self) ->NumType;
    fn magnitude(&self) -> f64;
    //fn add(&mut self, v : Vector2f64) -> Vector2f64;
    //fn subtract(&mut self, v : Vector2f64) -> Vector2f64;
    fn multiply(self, k :NumType) -> Self;
    fn multiply_assign(&mut self, k : NumType);
    fn set_xy(&mut self, x :NumType, y : NumType);
    fn set(&mut self, value :NumType);
}