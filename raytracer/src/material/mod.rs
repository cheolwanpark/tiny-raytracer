use crate::{hittable::HitRecord, math::vec3::Vec3, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub mod lambertian;
pub mod metal;
