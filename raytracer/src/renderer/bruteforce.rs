use super::{ColorSampler, ImageOptions, Renderer};
use crate::{camera::Camera, hittable::Hittable, image::Image, math::vec3::Vec3, random::random_float, Float};

pub struct BruteForceRenderer {
    pub color_sampler: Box<dyn ColorSampler>,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl BruteForceRenderer {
    pub fn new(color_sampler: Box<dyn ColorSampler>, samples_per_pixel: u32, max_depth: u32) -> BruteForceRenderer {
        BruteForceRenderer { color_sampler, samples_per_pixel, max_depth }
    }
}

impl Renderer for BruteForceRenderer {
    fn render(&self, camera: Camera, world: Box<dyn Hittable>, image_options: ImageOptions) -> Image {
        let mut image = Image::new(image_options.width, image_options.height);

        for j in 0..image_options.height {
            for i in 0..image_options.width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let u = (i as Float + random_float()) / (image_options.width - 1) as Float;
                    let v = (j as Float + random_float()) / (image_options.height - 1) as Float;

                    let ray = camera.get_ray(u, v);

                    pixel_color += self.color_sampler.sample(ray, &world, self.max_depth) / self.samples_per_pixel as Float;
                }

                image.set_pixel(i, j, pixel_color.into());
            }
        }

        image
    }
}

#[cfg(test)]
mod test {
    use crate::{hittable::{list::HittableList, sphere::Sphere}, renderer::colorsampler::NormalSampler};

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

        let renderer = BruteForceRenderer::new(Box::new(NormalSampler::new()), 100, 50);
        let image = renderer.render(camera, world, image_options);
        image.save("output/bruteforce.png")
    }
}