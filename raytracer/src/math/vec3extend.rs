use crate::{random::random_float_range, Float};
use std::ops::Range;

use super::vec3::Vec3;

impl Vec3 {
    pub fn new_random_range(range: Range<Float>) -> Self {
        Vec3::new(
            random_float_range(range.clone()),
            random_float_range(range.clone()),
            random_float_range(range),
        )
    }

    pub fn new_random_in_unit_sphere() -> Self {
        let mut p = Vec3::new_random_range(-1.0..1.0);
        while p.squared_length() >= 1.0 {
            p = Vec3::new_random_range(-1.0..1.0);
        }
        p
    }

    pub fn new_random_unit_vector() -> Self {
        Self::new_random_in_unit_sphere().normalized()
    }

    pub fn new_random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::new_random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn new_random() -> Self {
        Self::new_random_range(0.0..1.0)
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self.clone() - 2.0 * self.dot(normal) * normal.clone()
    }

    pub fn refract(&self, normal: Self, etai_over_etat: Float) -> Self {
        let cos = (-normal.dot(self)).min(1.0);
        let perp = etai_over_etat * (*self + normal*cos);
        let parallel = -(Float::from(1.0) - perp.squared_length()).abs().sqrt() * normal;
        parallel + perp
    }
}