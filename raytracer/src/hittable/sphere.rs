use std::{ops::Range, sync::Arc};

use crate::{accel::cpu::aabb::AABB, material::Material, math::vec3::Vec3, ray::Ray, Float};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: Float,
    bbox: AABB,
    material: Arc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: Arc<Box<dyn Material>>) -> Sphere {
        let radius_vec = Vec3::new_diagonal(radius);
        let bbox = AABB::new(center - radius_vec, center + radius_vec);
        Sphere {
            center,
            radius,
            bbox,
            material: material,
        }
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
            return None;
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
            p - self.center,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[cfg(test)]
mod tests {
    use std::f32::INFINITY;

    use super::*;
    use crate::material::lambertian::Lambertian;

    #[test]
    fn test_sphere_hit() {
        let dummy_mat: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian::new(Vec3::zero())));
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, dummy_mat.clone());
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
            assert!(hit_record.t - 3_f32.sqrt() / 2.0 < 1e-2);
            assert!(
                (hit_record.point - Vec3::new(0.0, 3_f32.sqrt() / 4.0, -3.0 / 4.0)).length() < 1e-2
            );
            assert!(
                (hit_record.normal - Vec3::new(0.0, 3_f32.sqrt(), 1.0).normalized()).length()
                    < 1e-2
            );
        } else {
            assert!(false, "Expected hit, but got None");
        }

        // Test ray that does not hit the sphere
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, -1.0));
        let hit = sphere.hit(&ray, 0.0..INFINITY);
        assert!(hit.is_none());
    }
}
