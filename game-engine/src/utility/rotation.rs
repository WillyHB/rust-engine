use std::f32::consts::PI;

pub fn degree_from_radians(angle : f32) -> f32 {

    angle * (180.0 / PI)
}

pub fn radians_from_degree(angle : f32) -> f32 {
    angle * (PI / 180.0)
}
