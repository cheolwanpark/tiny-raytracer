use crate::{math::vec3::Vec3, ray::Ray, Float};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: Float,
}

pub mod sphere;