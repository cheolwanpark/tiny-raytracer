use std::ops::Range;

use crate::{math::vec3::Vec3, ray::Ray, Float};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: Float,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float) -> Sphere {
        Sphere { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> Float {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<Float>) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = oc.dot(&ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        }
        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd) / a;
        if !t_range.contains(&t) {
            t = (-half_b + sqrtd) / a;
            if !t_range.contains(&t) {
                return None;
            }
        }
        
        let p = ray.at(t);
        Some(HitRecord::new(
            ray,
            t,
            p - self.center
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::f32::INFINITY;

    use super::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        if let Some(hit_record) = sphere.hit(&ray, 0.0..INFINITY) {
            assert_eq!(hit_record.t, 0.5);
            assert_eq!(hit_record.point, Vec3::new(0.0, 0.0, -0.5));
            assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, 1.0));
        } else {
            assert!(false, "Expected hit, but got None");
        }

        // Test ray that is not parallel with axis
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 3_f32.sqrt(), -3.0));
        if let Some(hit_record) = sphere.hit(&ray, 0.0..INFINITY) {
            assert!(hit_record.t - 3_f32.sqrt()/2.0 < 1e-2);
            assert!((hit_record.point - Vec3::new(0.0, 3_f32.sqrt()/4.0, -3.0/4.0)).length() < 1e-2);
            assert!((hit_record.normal - Vec3::new(0.0, 3_f32.sqrt(), 1.0).normalized()).length() < 1e-2);
        } else {
            assert!(false, "Expected hit, but got None");
        }

        // Test ray that does not hit the sphere
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, -1.0));
        let hit = sphere.hit(&ray, 0.0..INFINITY);
        assert!(hit.is_none());
    }
}