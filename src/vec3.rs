use crate::rtweekend::random_float;
use crate::rtweekend::random_floats;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self::Output {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f64 {
        return self.square_length().sqrt();
    }

    pub fn square_length(self) -> f64 {
        let new = self * self;
        return new.x + new.y + new.z;
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        };
    }

    pub fn normalize(&self) -> Vec3 {
        let inv_len = Vec3::length(*self).recip();
        Vec3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        let new = v1 * v2;
        return new.x + new.y + new.z;
    }

    pub fn unit_vector(self) -> Vec3 {
        return self / self.length();
    }
    pub fn random_vec3() -> Vec3 {
        return Vec3 {
            x: (random_float()),
            y: (random_float()),
            z: (random_float()),
        };
    }
    pub fn random_vec3s(min: f64, max: f64) -> Vec3 {
        return Vec3 {
            x: (random_floats(min, max)),
            y: (random_floats(min, max)),
            z: (random_floats(min, max)),
        };
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_vec3s(-1.0, 1.0);
            if p.square_length() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        return Self::random_in_unit_sphere().unit_vector();
    }

    pub fn random_in_hemisphere(v: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(in_unit_sphere, v) > 0.0 {
            return in_unit_sphere;
        } else {
            return -(in_unit_sphere);
        }
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }
    pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
        return v - (normal * 2.0 * Self::dot(v, normal));
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
