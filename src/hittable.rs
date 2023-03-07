use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use crate::vec3::Vec3;
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

enum Hittable {
    S(Sphere),
}

impl Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
