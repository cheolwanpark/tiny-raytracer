use crate::{hittable::HitRecord, math::vec3::Vec3, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;

    fn emitted(&self) -> Option<Vec3> {
        None
    }
}

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod light;
