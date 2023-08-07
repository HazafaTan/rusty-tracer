use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;

pub enum Material {
    Lambertian(Color),
    Metal { color: Color, fuzz: f64 },
    Dielectric { ir: f64 },
}

impl Material {
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
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
                *scattered = Ray::new(scatter_direction,rec.p );
                *attenuation = *albedo;
                return true;
            }
            Material::Metal {
                color: albedo,
                fuzz,
            } => {
                let reflected = Vec3::reflect(r.direction, rec.normal);
                *scattered = Ray::new(reflected + fuzz * Vec3::random_unit_vector(), rec.p );
                *attenuation = *albedo;
                return Vec3::dot(scattered.direction, rec.normal) > 0.0;
            }
            Material::Dielectric{
                ir
            } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face { 1.0 / ir } else { *ir };
                let unit_direction = r.direction.unit_vector();
                let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction;
                if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>(){
                    direction = Vec3::reflect(unit_direction, rec.normal);
                } else {
                    direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
                }
                *scattered = Ray::new(direction, rec.p );
                return true;
            }
        }
    }
}
