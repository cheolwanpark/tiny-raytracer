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
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        unimplemented!()
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
        let hit = sphere.hit(&ray, 0.0, INFINITY);
        assert!(hit.is_some());
        
        if let Some(hit_record) = sphere.hit(&ray, 0.0, INFINITY) {
            assert_eq!(hit_record.t, 0.5);
            assert_eq!(hit_record.p, Vec3::new(0.0, 0.0, -0.5));
            assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, 1.0));
        } else {
            assert!(false, "Expected hit, but got None");
        }

        // Test ray that is not parallel with axis
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 3_f32.sqrt(), -3.0));
        let hit = sphere.hit(&ray, 0.0, INFINITY);
        assert!(hit.is_some());

        if let Some(hit_record) = sphere.hit(&ray, 0.0, INFINITY) {
            assert_eq!(hit_record.t, 0.5);
            assert_eq!(hit_record.p, Vec3::new(0.0, 3_f32.sqrt()/4.0, -3.0/4.0));
            assert_eq!(hit_record.normal, Vec3::new(0.0, 3_f32.sqrt(), 1.0).normalize());
        } else {
            assert!(false, "Expected hit, but got None");
        }

        // Test ray that does not hit the sphere
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, -1.0));
        let hit = sphere.hit(&ray, 0.0, INFINITY);
        assert!(hit.is_none());
    }
}