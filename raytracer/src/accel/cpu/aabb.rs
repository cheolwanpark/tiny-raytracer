use std::{cmp::Ordering, mem::swap, ops::Range};

use crate::{math::vec3::Vec3, ray::Ray, Float, Int};

#[derive(Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        const PADDING_AMOUNT: Float = 0.0001;
        let padding = Vec3::new_diagonal(PADDING_AMOUNT / 2.0);

        let min = Vec3::new_min(a, b) - padding;
        let max = Vec3::new_max(a, b) + padding;
        Self { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn merge(a: AABB, b: AABB) -> Self {
        let min = Vec3::new_min(a.min, b.min);
        let max = Vec3::new_max(a.max, b.max);
        Self { min, max }
    }

    pub fn intersect(&self, ray: &Ray, mut t_range: Range<Float>) -> bool {
        let o = ray.origin();
        let d = ray.direction();
        for i in 0..3 {
            let min = self.min[i];
            let max = self.max[i];
            let inv_d = Float::from(1.0) / d[i];

            let mut t0 = (min - o[i]) * inv_d;
            let mut t1 = (max - o[i]) * inv_d;
            if t1 < t0 {
                swap(&mut t0, &mut t1);
            }
            if t_range.start < t0 {
                t_range.start = t0;
            }
            if t1 < t_range.end {
                t_range.end = t1;
            }

            if t_range.end <= t_range.start {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        let sizes: Vec<Float> = (0..3).map(|i| self.max[i] - self.min[i]).collect();
        if sizes[0] > sizes[1] {
            if sizes[0] > sizes[2] {
                0
            } else {
                2
            }
        } else {
            if sizes[1] > sizes[2] {
                1
            } else {
                2
            }
        }
    }

    pub fn compare(a: AABB, b: AABB, axis: usize) -> Ordering {
        a.min[axis].total_cmp(&b.min[axis])
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Vec3::zero(),
            max: Vec3::zero(),
        }
    }
}