use crate::{utils::random::{random, random_range}, Float};
use std::ops::Range;

use super::vec3::Vec3;

impl Vec3 {
    pub fn new_random_range(range: Range<Float>) -> Self {
        Vec3::new(
            random_range(range.clone()),
            random_range(range.clone()),
            random_range(range),
        )
    }

    pub fn new_random_in_unit_sphere() -> Self {
        let u1: Float = random();
        let u2: Float = random();
        let u3: Float = random();
        
        let theta = 2.0 * std::f32::consts::PI * u1;
        let phi = (1.0 - 2.0 * u2).acos(); // Invert to get uniform distribution
        let r = u3.cbrt(); // Cube root for uniform distribution
        
        let sin_phi = phi.sin();
        let x = r * sin_phi * theta.cos();
        let y = r * sin_phi * theta.sin();
        let z = r * phi.cos();
        
        Vec3::new(x, y, z)
    }

    pub fn new_random_unit_vector() -> Self {
        Self::new_random_in_unit_sphere().normalized()
    }

    pub fn new_random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::new_random_unit_vector();
        if on_unit_sphere.dot(normal) > Float::from(0.0) {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn new_random_in_unit_disk() -> Self {
        loop {
            let range = Float::from(-1.0)..Float::from(1.0);
            let p = Vec3::new(random_range(range.clone()), random_range(range), 0.0);
            if p.squared_length() < Float::from(1.0) {
                return p
            }
        }
    }

    pub fn new_random() -> Self {
        Self::new_random_range(Float::from(0.0)..Float::from(1.0))
    }

    pub fn new_min(a: Vec3, b: Vec3) -> Self {
        Self { 
            x: a.x.min(b.x), 
            y: a.y.min(b.y),
            z: a.z.min(b.z),
        }
    }

    pub fn new_max(a: Vec3, b: Vec3) -> Self {
        Self { 
            x: a.x.max(b.x), 
            y: a.y.max(b.y),
            z: a.z.max(b.z),
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self.clone() - Float::from(2.0) * self.dot(normal) * normal.clone()
    }

    pub fn refract(&self, normal: Self, etai_over_etat: Float) -> Self {
        let cos = (-normal.dot(self)).min(Float::from(1.0));
        let perp = etai_over_etat * (*self + normal*cos);
        let parallel = -(Float::from(1.0) - perp.squared_length()).abs().sqrt() * normal;
        parallel + perp
    }
}