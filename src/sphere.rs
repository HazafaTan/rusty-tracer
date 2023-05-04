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
        let mut rec = HitRecord {
            t: root,
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            mat: &self.mat,
        };
        let outward_normal = Vec3::divide(Vec3::sub(p, self.center), self.radius);
        rec.set_face_normal(*r, outward_normal);
        Some(rec)
    }
}
