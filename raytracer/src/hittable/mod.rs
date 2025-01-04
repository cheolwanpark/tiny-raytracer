use std::{ops::Range, sync::Arc};

use as_any::AsAny;

use crate::{material::Material, math::vec3::Vec3, ray::Ray, Float};

pub mod list;
pub mod world;
pub mod sphere;
pub mod quad;
pub mod aabb;
pub mod bvh;

pub trait Hittable: AsAny {
    fn hit(&self, ray: &Ray, t_range: Range<Float>) -> Option<HitRecord>;

    fn bounding_box(&self) -> aabb::AABB;
}

pub struct HitRecord {
    pub t: Float,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        t: Float,
        outward_normal: Vec3,
        material: Arc<Box<dyn Material>>,
    ) -> HitRecord {
        let point = ray.at(t);
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.normalized()
        } else {
            -outward_normal.normalized()
        };
        HitRecord {
            t,
            point,
            normal,
            front_face,
            material,
        }
    }
}