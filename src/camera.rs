use crate::ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Default for Camera {
    fn default() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = Vec3::sub(
            Vec3::sub(
                Vec3::sub(origin, Vec3::divide(horizontal, 2.0)),
                Vec3::divide(vertical, 2.0),
            ),
            Vec3 {
                x: (0.0),
                y: (0.0),
                z: (focal_length),
            },
        );
        Camera {
            origin: (origin),
            horizontal: (horizontal),
            vertical: (vertical),
            lower_left_corner: (lower_left_corner),
        }
    }
}

impl Camera {
    fn new(origin: Vec3, horizontal: Vec3, vertical: Vec3, lower_left_corner: Vec3) -> Camera {
        Camera {
            origin: (origin),
            horizontal: (horizontal),
            vertical: (vertical),
            lower_left_corner: (lower_left_corner),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::Ray::new(
            Vec3::add(
                Vec3::add(
                    Vec3::multiply(self.horizontal, u),
                    Vec3::sub(Vec3::multiply(self.vertical, v), self.origin),
                ),
                self.lower_left_corner,
            ),
            self.origin,
        )
    }
}

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
