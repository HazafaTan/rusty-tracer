use crate::vec3;
use crate::vec3::Vec3;

pub struct Ray {
    direction: vec3::Vec3,
    origin: vec3::point3,
}

impl Ray {
    pub fn new(direction: vec3::Vec3, origin: vec3::point3) -> Ray {
        return Ray { direction, origin };
    }

    pub fn origin(r: Ray) -> Vec3 {
        r.origin
    }
    pub fn direction(r: Ray) -> Vec3 {
        r.direction
    }
    pub fn at(t: f64, r: Ray) -> Vec3 {
        Vec3::add(r.origin, Vec3::multiply(r.direction, t))
    }
}
