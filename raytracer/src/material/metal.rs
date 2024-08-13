use crate::{hittable::HitRecord, math::vec3::Vec3, ray::Ray, Float};

use super::Material;

pub struct Metal {
    albedo: Vec3,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: Float) -> Metal {
        Metal { albedo, fuzz: fuzz.clamp(Float::from(0.0), Float::from(1.0)) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * Vec3::new_random_in_unit_sphere(),
        );
        Some((scattered, self.albedo.clone()))
    }
}
