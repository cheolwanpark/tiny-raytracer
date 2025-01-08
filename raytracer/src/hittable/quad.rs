use std::{ops::Range, sync::Arc};

use crate::{material::Material, math::vec3::Vec3, ray::Ray, Float};

use super::{aabb::AABB, HitRecord, Hittable};

#[derive(Clone)]
pub struct Quad {
    pub corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub n: Vec3,
    pub w: Vec3,
    pub d: Float,
    bbox: AABB,
    pub material: Arc<Box<dyn Material>>,
}

impl Quad {
    pub fn new(corner: Vec3, u: Vec3, v: Vec3, material: Arc<Box<dyn Material>>) -> Self {
        let bbox = AABB::merge(
            AABB::new(corner, corner + u + v),
            AABB::new(corner + u, corner + v)
        );
        let n = u.cross(&v);
        let w = n / n.dot(&n);
        let d = n.dot(&corner);
        Self { corner, u, v, n, w, d, bbox, material } 
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_range: Range<Float>) -> Option<HitRecord> {
        let dir_norm = ray.direction().dot(&self.n);

        let t = (self.d - ray.origin().dot(&self.n)) / dir_norm;
        if t_range.contains(&t) {
            let p = ray.at(t) - self.corner;
            let planar_x = p.cross(&self.v).dot(&self.w);
            let planar_y = self.u.cross(&p).dot(&self.w);
            if (0.0..1.0).contains(&planar_x) && (0.0..1.0).contains(&planar_y) {
                Some(HitRecord::new(
                    ray,
                    t,
                    self.n,
                    self.material.clone(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[cfg(test)]
mod tests {
    use crate::{camera::Camera, hittable::world::World, material::lambertian::Lambertian, renderer::Renderer};

    use super::*;

    #[test]
    fn test_quad_hit() {
        let quad = Quad::new(
            Vec3::zero(),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 3.0),
            Arc::new(Box::new(Lambertian::new(Vec3::zero())))
        );

        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        if let Some(hit_rec) = quad.hit(&ray, 0.0..Float::INFINITY) {
            assert_eq!(hit_rec.t, 1.0);
            assert_eq!(hit_rec.point, Vec3::new(0.0, 0.0, 0.0));
            assert_eq!(hit_rec.normal, Vec3::new(0.0, 1.0, 0.0));
        } else {
            assert!(false, "Expected hit, but got None");
        }

        let ray = Ray::new(Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, -1.0, -1.0));
        if let Some(hit_rec) = quad.hit(&ray, 0.0..Float::INFINITY) {
            assert!((hit_rec.t -2.0_f32.sqrt()) < 1e-6);
            assert_eq!(hit_rec.point, Vec3::new(0.0, 0.0, 0.0));
            assert_eq!(hit_rec.normal, Vec3::new(0.0, 1.0, 0.0));
        } else {
            assert!(false, "Expected hit, but got None");
        }

        let ray = Ray::new(Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, -1.0, -1.1));
        assert!(quad.hit(&ray, 0.0..Float::INFINITY).is_none());
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads=4)]
    async fn test_rendering() {
        let mut world = World::new();

        world.add_material("red", Box::new(Lambertian::new(Vec3::new(1.0, 0.2, 0.2))));
        world.add_material("green", Box::new(Lambertian::new(Vec3::new(0.2, 1.0, 0.2))));
        world.add_material("blue", Box::new(Lambertian::new(Vec3::new(0.2, 0.2, 1.0))));
        world.add_material("orange", Box::new(Lambertian::new(Vec3::new(1.0, 0.5, 0.0))));
        world.add_material("teal", Box::new(Lambertian::new(Vec3::new(0.2, 0.8, 0.8))));

        world.add_geometry(Box::new(Quad::new(
            Vec3::new(-3.0, -2.0, 5.0),
            Vec3::new(0.0, 0.0, -4.0),
            Vec3::new(0.0, 4.0, 0.0),
            world.get_material("red").unwrap()
        )));
        world.add_geometry(Box::new(Quad::new(
            Vec3::new(-2.0, -2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            world.get_material("green").unwrap()
        )));
        world.add_geometry(Box::new(Quad::new(
            Vec3::new(3.0, -2.0, 1.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 4.0, 0.0),
            world.get_material("blue").unwrap()
        )));
        world.add_geometry(Box::new(Quad::new(
            Vec3::new(-2.0, 3.0, 1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
            world.get_material("orange").unwrap()
        )));
        world.add_geometry(Box::new(Quad::new(
            Vec3::new(-2.0, -3.0, 5.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -4.0),
            world.get_material("teal").unwrap()
        )));

        let camera = Camera::new(
            1.0,
            0.0,
            Vec3::new(0.0, 0.0, 9.0),
            Vec3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            80.0,
            400, 300
        );
        let instance = Renderer::new(10, 4, 10, false, Some(Vec3::new(0.7, 0.8, 1.0)));
        instance.render(camera, Arc::new(world)).await.expect("failed to create image").save("output/quad_test.png");
    }
}