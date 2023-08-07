use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a Material,
}

impl HitRecord<'_> {
    /*
     pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) -> () {
        self.front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
    }
     */
   
}
pub enum Hittable {
    S(Sphere),
}
impl Hittable {
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::S(sphere) => sphere.hit(&r, t_min, t_max),
        }
    }
}
pub struct HittableList {
    pub objects: Vec<Hittable>,
}

impl HittableList {
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for hittable in &self.objects {
            if let Some(rec) = hittable.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
