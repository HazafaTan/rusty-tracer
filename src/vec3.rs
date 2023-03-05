use std::fmt;

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
pub type color = Vec3;
pub type point3 = Vec3;
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

    pub fn div(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x / v2.x,
            y: v1.y / v2.y,
            z: v1.z / v2.z,
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
    pub fn addition(v1: Vec3, t: f64) -> Vec3 {
        return Vec3 {
            x: v1.x + t,
            y: v1.y + t,
            z: v1.z + t,
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
    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        return Self::divide(v, Self::length(v));
    }
}
