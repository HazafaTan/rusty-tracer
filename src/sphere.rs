use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Material,
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.square_length();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.square_length() - (self.radius * self.radius);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = Ray::at(root, *r);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = Vec3::dot(outward_normal, r.direction) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord {
            t: root,
            p,
            normal,
            front_face,
            mat: &self.mat, // Assuming self.mat is the material of the sphere
        })
    }
}
