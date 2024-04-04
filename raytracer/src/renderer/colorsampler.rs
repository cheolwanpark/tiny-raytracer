use crate::{hittable::Hittable, math::vec3::Vec3, Float};

use super::ColorSampler;

pub struct NormalSampler;

impl NormalSampler {
    pub fn new() -> Self {
        NormalSampler {}
    }
}

impl ColorSampler for NormalSampler {
    fn sample(&self, ray: crate::ray::Ray, world: &Box<dyn Hittable>, depth: u32) -> Vec3 {
        if let Some(rec) = world.hit(&ray, 0.0..Float::INFINITY) {
            0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0))
        } else {
            let direction = ray.direction();
            let a = 0.5*(direction.y + 1.0);
            return (1.0-a)*Vec3::new(1.0, 1.0, 1.0) + a*Vec3::new(0.5, 0.7, 1.0);
        }
        
    }
}

pub struct GeneralSampler;

impl GeneralSampler {
    pub fn new() -> Self {
        GeneralSampler {}
    }
}

impl ColorSampler for GeneralSampler {
    fn sample(&self, ray: crate::ray::Ray, world: &Box<dyn Hittable>, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::zero()
        }

        if let Some(rec) = world.hit(&ray, 0.001..Float::INFINITY) {
            let direction = rec.normal + Vec3::new_random_unit_vector();
            0.5 * self.sample(crate::ray::Ray::new(rec.point, direction), world, depth - 1)
        } else {
            let direction = ray.direction();
            let a = 0.5*(direction.y + 1.0);
            return (1.0-a)*Vec3::new(1.0, 1.0, 1.0) + a*Vec3::new(0.5, 0.7, 1.0);
        }
    }
}