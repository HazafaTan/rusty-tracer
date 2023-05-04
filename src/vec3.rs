use std::fmt;
use std::ops::Neg;

use crate::rtweekend::random_float;
use crate::rtweekend::random_floats;

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

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(v: Vec3) -> f64 {
        return Self::square_length(v).sqrt();
    }

    pub fn square_length(v: Vec3) -> f64 {
        return v.x * v.x + v.y * v.y + v.z * v.z;
    }

    pub fn add(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
        };
    }

    pub fn times(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x * v2.x,
            y: v1.y * v2.y,
            z: v1.z * v2.z,
        };
    }
    pub fn multiply(v1: Vec3, t: f64) -> Vec3 {
        return Vec3 {
            x: v1.x * t,
            y: v1.y * t,
            z: v1.z * t,
        };
    }
    pub fn divide(v1: Vec3, t: f64) -> Vec3 {
        return Vec3 {
            x: v1.x / t,
            y: v1.y / t,
            z: v1.z / t,
        };
    }

    pub fn sub(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: (v1.x - v2.x),
            y: (v1.y - v2.y),
            z: (v1.z - v2.z),
        };
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
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        return Self::divide(v, Self::length(v));
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
            if Self::square_length(p) >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        return Self::unit_vector(Self::random_in_unit_sphere());
    }

    pub fn random_in_hemisphere(v: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(in_unit_sphere, v) > 0.0 {
            return in_unit_sphere;
        } else {
            return -(in_unit_sphere);
        }
    }
    pub fn near_zero(v: Vec3) -> bool {
        let s = 1e-8;
        return (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s);
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        let dot = 2.0 * (Self::dot(v, n));
        let c = Self::multiply(n, dot);
        let j = Self::sub(v, c);
        return j;
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
