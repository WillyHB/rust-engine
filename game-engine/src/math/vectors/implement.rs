/// A macro to implement a Vector2 on various data types
#[macro_export]
macro_rules! impl_vector2 {

    ($struct_name:ident, $t : ty) => {
        #[derive(Copy, Clone)]
        pub struct $struct_name {

            pub x : $t,
            pub y : $t,
        }

        impl Vector2<$t> for $struct_name {

            #[doc = concat!("Creates a new vector of type `", stringify!($t), "`.")]
            fn new(x:$t,y:$t) -> Self {

                Self {x,y}
            }

            #[doc = concat!("Return the zero vector for the type `", stringify!($t), "`.")]
            fn zero() -> Self {

                Self {x:0 as $t,y:0 as $t}
            }

            #[doc = concat!("Returns a vector with all values `1` of type `", stringify!($t), "`.")]
            fn one() -> Self {

                Self {x:1 as $t,y:1 as $t}
            }

            /// Rturns the dot product of the vector
            fn dot (v1 : Self, v2 : Self) -> $t{

                v1.x * v2.x + v1.y * v2.y
            }

            
            #[doc = "Return the magnitude of the vector, with type `f32`."]
            fn magnitude(&self) -> f64 {

                ((self.x as f64*self.x as f64) + (self.y as f64*self.y as f64)).sqrt()
            }

            #[doc = "Returns the vector multiplied by the scalar `k`."]
            fn multiply(self, k : $t) -> Self {

                Self{
                x : self.x * k,
                y : self.y * k,
                }

            }


            #[doc = "Multiplies `k` to the vector and assigns it the value"]
            fn multiply_assign(&mut self, k : $t) {

                self.x *= k;
                self.y *= k;
            }

            #[doc = "Assigns `x` and `y` of the vector to the parameters"]
            fn set_xy(&mut self, x : $t, y : $t) {

                self.x = x;
                self.y = y;
            }

            #[doc = "Assigns both `x` and `y` of the vector to the parameter"]
            fn set(&mut self, value : $t) {

                self.x = value;
                self.y = value;
            }
        }

        impl fmt::Display for $struct_name {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        
        impl std::ops::Add for $struct_name {

            type Output = Self;
     
            fn add(self, other : Self) -> Self {

                Self { x : self.x + other.x, y : self.y + other.y }
            }
        }

        impl std::ops::Sub for $struct_name {

            type Output = Self;
     
            fn sub(self, other : Self) -> Self::Output {

                Self { x : self.x - other.x, y : self.y - other.y }
            }
        }

        impl std::ops::AddAssign for $struct_name {

            fn add_assign(&mut self, rhs: Self) {
        
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl std::ops::SubAssign for $struct_name {

            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }
    };
}

pub use impl_vector2;