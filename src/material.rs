use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;

pub enum Material {
    Lambertian(Color),
    Metal { color: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter(
        &self,
        r: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;
                return true;
            }
            Material::Metal {
                color: albedo,
                fuzz: _,
            } => {
                let reflected = Vec3::reflect(r.direction, rec.normal);
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = *albedo;
                return Vec3::dot(scattered.direction, rec.normal) > 0.0;
            }
        }
    }
}
