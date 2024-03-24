use crate::{math::vec3::Vec3, ray::Ray, Float};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: Float,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: Float, outward_normal: Vec3) -> HitRecord {
        let point = ray.at(t);
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.normalized()
        } else {
            -outward_normal.normalized()
        };
        HitRecord { t, point, normal, front_face }
    }
}

pub mod sphere;
pub mod list;