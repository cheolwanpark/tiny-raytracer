use crate::{hittable::HitRecord, math::vec3::Vec3, utils::random::random, ray::Ray, Float};

use super::Material;

pub struct Dielectric {
    albedo: Vec3,
    refraction_index: Float,
}

impl Dielectric {
    pub fn new(albedo: Vec3, refraction_index: Float) -> Self {
        Dielectric { albedo, refraction_index }
    }

    fn reflectance(cos: Float, refraction_index: Float) -> Float {
        // Schlick's approximation
        let one = Float::from(1.0);
        let sqrt_r0 = (one - refraction_index) / (one + refraction_index);
        let r0 = sqrt_r0 * sqrt_r0;
        r0 + (one - r0)*(one - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_index = if hit_record.front_face {
            Float::from(1.0) / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos = (-hit_record.normal.dot(&ray.direction())).min(1.0);
        let sin = (Float::from(1.0) - cos*cos).sqrt();

        let total_reflection = refraction_index * sin > 1.0;
        let reflectance = Self::reflectance(cos, refraction_index);
        let direction = if total_reflection || reflectance > random() {
            ray.direction().reflect(&hit_record.normal)
        } else {
            ray.direction().refract(hit_record.normal, refraction_index)
        };

        let refracted_ray = Ray::new(hit_record.point, direction);
        Some((refracted_ray, self.albedo))
    }
}