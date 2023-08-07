use crate::ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom:Point3, lookat:Point3, vup:Vec3, vfov: f64, aspect_ratio: f64, defocus_angle:f64, focus_dist:f64) -> Camera {

        let theta = PI / 180.0 * vfov;
        let viewport_height:f64 = 2.0 * (theta / 2.0).tan();
        let viewport_width:f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = Vec3::cross(vup, w).normalize();
        let v = Vec3::cross(w, u);

        let horizontal =   (u* viewport_width) * focus_dist;
        let vertical =   (v* viewport_height) * focus_dist;

        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - (w* focus_dist);

        Camera {
            origin: (lookfrom),
            horizontal: (horizontal),
            vertical: (vertical),
            lower_left_corner: (lower_left_corner),
            u: (u),
            v: (v),
            lens_radius: defocus_angle / 2.0
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        ray::Ray::new(
            self.lower_left_corner + ( self.horizontal*u) + (self.vertical * v) - self.origin - offset,
            self.origin +offset,
        )
    }
}

use std::f64::consts::PI;
use std::fmt;

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Camera {{\n")?;
        write!(f, "  origin: {}\n", self.origin)?;
        write!(f, "  horizontal: {}\n", self.horizontal)?;
        write!(f, "  vertical: {}\n", self.vertical)?;
        write!(f, "  lower_left_corner: {}\n", self.lower_left_corner)?;
        write!(f, "}}")
    }
}
