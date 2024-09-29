#![allow(dead_code)]

use crate::src::misc::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Point3 = Vec3;

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /// get 3D vector form array
    pub fn from(a: &[f64; 3]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    // vector length
    pub fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
    pub fn len_sqrd(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    // get the vector normalized
    pub fn unit(&self) -> Self {
        let inverse_legth = 1.0 / self.len();
        Self {
            x: self.x * inverse_legth,
            y: self.y * inverse_legth,
            z: self.z * inverse_legth,
        }
    }
    /// linear interpolation between 2 vectors
    pub fn mix(&self, other: Self, c: f64) -> Self {
        (*self * (1.0 - c)) + (other * c)
    }

    pub fn random() -> Self {
        vec3(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        vec3(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn random_unit() -> Vec3 {
        random_in_unit_sphere().unit()
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.len_sqrd() >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}
// dot product with another vector
pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}
// reflect a vector around a normal
pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    *incident - 2.0 * dot(incident, normal) * *normal
}
//get the cross product between two vectors
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
}

pub fn clamp_vec3(v: &Vec3, min: &Vec3, max: &Vec3) -> Vec3 {
    Vec3::new(
        clamp(v.x, min.x, max.x),
        clamp(v.y, min.y, max.y),
        clamp(v.z, min.z, max.z),
    )
}
use std::ops::*;
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
