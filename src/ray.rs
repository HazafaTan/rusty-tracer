use crate::vec3;
use crate::vec3::Point3;
use crate::vec3::Vec3;
#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Ray {
    pub direction: vec3::Vec3,
    pub origin: vec3::Point3,
}

impl Ray {
    pub fn new(direction: Vec3, origin: Point3) -> Ray {
        return Ray { direction, origin };
    }

    pub fn at(t: f64, r: Ray) -> Vec3 {
        r.origin + (r.direction * t)
    }
}
