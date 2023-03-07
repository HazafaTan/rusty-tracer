use crate::hittable::HitRecord;
use crate::ray;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

fn hit_sphere(
    center: Point3,
    radius: f64,
    r: ray::Ray,
    t_min: f64,
    t_max: f64,
    mut rec: HitRecord,
) -> bool {
    let oc = Vec3::sub(r.origin, center);
    let a = Vec3::square_length(r.direction);
    let half_b = Vec3::dot(oc, r.direction);
    let c = Vec3::square_length(oc) - radius * radius;

    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        return false;
    }
    let sqrtd = discriminant.sqrt();
    //Find the nearest root that lies in the acceptable range
    let root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return false;
        }
    }
    rec.t = root;
    rec.p = Ray::at(root, r);
    rec.normal = Vec3::divide(Vec3::sub(rec.p, center), radius);

    return true;
}
