use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = Vec3::sub(r.origin, self.center);
        let a = Vec3::square_length(r.direction);
        let half_b = Vec3::dot(oc, r.direction);
        let c = Vec3::square_length(oc) - self.radius * self.radius;

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        //Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = Ray::at(root, *r);
        let outward_normal = Vec3::divide(Vec3::sub(p, self.center), self.radius);
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            Vec3::multiply(outward_normal, -1.0)
        };
        Some(HitRecord {
            p,
            normal,
            t: root,
            front_face,
        })
    }
}
