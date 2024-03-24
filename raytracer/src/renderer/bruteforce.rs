use super::{Renderer, ImageOptions};
use crate::{camera::Camera, hittable::{Hittable, list::HittableList, sphere::Sphere}, image::Image, math::vec3::Vec3, random::random_float, Float};

pub struct BruteForceRenderer {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl BruteForceRenderer {
    pub fn new(samples_per_pixel: u32, max_depth: u32) -> BruteForceRenderer {
        BruteForceRenderer { samples_per_pixel, max_depth }
    }
}

impl Renderer for BruteForceRenderer {
    fn ray_color(&self, ray: crate::ray::Ray, world: &Box<dyn Hittable>, depth: u32) -> Vec3 {
        if let Some(rec) = world.hit(&ray, 0.0, Float::INFINITY) {
            0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0))
        } else {
            let direction = ray.direction();
            let a = 0.5*(direction.y + 1.0);
            return (1.0-a)*Vec3::new(1.0, 1.0, 1.0) + a*Vec3::new(0.5, 0.7, 1.0);
        }
    }

    fn render(&self, camera: Camera, world: Box<dyn Hittable>, image_options: ImageOptions) -> Image {
        let mut image = Image::new(image_options.width, image_options.height);

        for j in 0..image_options.height {
            for i in 0..image_options.width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let u = (i as Float + random_float()) / (image_options.width - 1) as Float;
                    let v = (j as Float + random_float()) / (image_options.height - 1) as Float;

                    let ray = camera.get_ray(u, v);

                    pixel_color += self.ray_color(ray, &world, self.max_depth) / self.samples_per_pixel as Float;
                }

                image.set_pixel(i, j, pixel_color.into());
            }
        }

        image
    }
}

#[cfg(test)]
mod test {
    use crate::hittable::list::HittableList;

    use super::*;

    #[test]
    #[ignore]
    fn test_bruteforce_renderer() {
        let width = 400_usize;
        let aspect_ratio = 16.0 / 9.0;
        let image_options = ImageOptions::new(width, (width as Float / aspect_ratio) as usize);
        let camera = Camera::new(
            1.0, 
            Vec3::zero(), 
            Vec3::new(0.0, 0.0, -1.0), 
            Vec3::new(0.0, 1.0, 0.0),
             90.0, 
             aspect_ratio
        );

        let mut world = Box::new(HittableList::new());
        world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

        let renderer = BruteForceRenderer::new(100, 50);
        let image = renderer.render(camera, world, image_options);
        image.save("output/bruteforce.png")
    }
}