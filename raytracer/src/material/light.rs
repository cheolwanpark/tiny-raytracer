use crate::{hittable::HitRecord, math::vec3::Vec3, ray::Ray};

use super::Material;

pub struct Light {
    color: Vec3,
}

impl Light {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for Light {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self) -> Option<Vec3> {
        Some(self.color)
    }
}