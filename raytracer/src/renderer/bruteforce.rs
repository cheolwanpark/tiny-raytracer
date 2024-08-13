use indicatif::ProgressBar;

use super::{ColorSampler, ImageOptions, Renderer};
use crate::{
    camera::Camera, hittable::Hittable, image::Image, math::vec3::Vec3, random::random_float, Float,
};

pub struct BruteForceRenderer {
    color_sampler: Box<dyn ColorSampler>,
    samples_per_pixel: u32,
    max_depth: u32,
    verbose: bool,
}

impl BruteForceRenderer {
    pub fn new(
        color_sampler: Box<dyn ColorSampler>,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> BruteForceRenderer {
        BruteForceRenderer {
            color_sampler,
            samples_per_pixel,
            max_depth,
            verbose: true,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }
}

impl Renderer for BruteForceRenderer {
    fn render(
        &self,
        camera: Camera,
        world: Box<dyn Hittable>,
        image_options: ImageOptions,
    ) -> Image {
        let mut image =
            Image::new_with_gamma_correction(image_options.width, image_options.height, 2.2);

        let progressbar = if self.verbose {
            Some(ProgressBar::new(
                image_options.width as u64 * image_options.height as u64,
            ))
        } else {
            None
        };

        for j in 0..image_options.height {
            for i in 0..image_options.width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let u = (i as Float + random_float()) / (image_options.width - 1) as Float;
                    let v = (j as Float + random_float()) / (image_options.height - 1) as Float;

                    let ray = camera.get_ray(u, v);

                    pixel_color += self.color_sampler.sample(ray, &world, self.max_depth)
                        / self.samples_per_pixel as Float;
                }

                image.set_pixel(i, j, pixel_color.into());
                if let Some(bar) = &progressbar {
                    bar.inc(1);
                }
            }
        }

        if let Some(bar) = &progressbar {
            bar.finish();
        }

        image
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{
        hittable::{list::HittableList, sphere::Sphere},
        material::{lambertian::Lambertian, metal::Metal, Material},
        renderer::colorsampler::GeneralSampler,
    };

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
            aspect_ratio,
        );

        let mat_ground: Rc<Box<dyn Material>> =
            Rc::new(Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))));
        let mat_center: Rc<Box<dyn Material>> =
            Rc::new(Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))));
        let mat_left: Rc<Box<dyn Material>> =
            Rc::new(Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)));
        let mat_right: Rc<Box<dyn Material>> =
            Rc::new(Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)));

        let mut world = Box::new(HittableList::new());
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            mat_ground,
        )));
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.2),
            0.5,
            mat_center,
        )));
        
        world.push(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            mat_left,
        )));
        world.push(Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            mat_right,
        )));

        let renderer = BruteForceRenderer::new(Box::new(GeneralSampler::new()), 50, 10);
        let image = renderer.render(camera, world, image_options);
        image.save("output/bruteforce.png")
    }
}
