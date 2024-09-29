#![allow(dead_code)]

pub const PIE: f32 = 3.1415927;

pub use std::f64::INFINITY;

use rand::Rng;

pub fn radians(v: f32) -> f32 {
    v * (PIE / 180.0)
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn minimum(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn maximum(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn clamp(v: f64, min: f64, max: f64) -> f64 {
    maximum(min, minimum(v, max))
}

pub fn step(a: f32, b: f32) -> i32 {
    if b < a {
        return 0;
    } else {
        return 1;
    }
}
