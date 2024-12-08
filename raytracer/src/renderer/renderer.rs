use std::sync::Arc;

use flume::bounded;
use indicatif::ProgressBar;
use tokio::{self, task::JoinHandle};

use crate::{camera::Camera, hittable::world::World, math::vec3::Vec3, utils::image::Image};

use super::{imager::Imager, pointgen::SamplePointGenerator, sampler::Sampler};

#[derive(Clone, Copy)]
pub struct Renderer {
    samples_per_pixel: usize,
    num_sampler_threads: usize,
    max_bounces: usize,
    progressbar: bool,
    background_color: Vec3,
}

impl Renderer {
    pub fn new(
        samples_per_pixel: usize,
        num_sampler_threads: usize,
        max_bounces: usize,
        progressbar: bool,
        background_color: Option<Vec3>,
    ) -> Self {
        Self { 
            samples_per_pixel,
            num_sampler_threads,
            max_bounces,
            progressbar,
            background_color: background_color.unwrap_or(Vec3::zero())
        }
    }

    pub fn render(&self, camera: Camera, world: Arc<World>) -> JoinHandle<Image> {
        let (width, height) = camera.get_image_size();
        let point_generator = SamplePointGenerator::new(
            width, 
            height, 
            self.samples_per_pixel,
            camera.clone()
        );
        let sampler = Sampler::new(
            self.num_sampler_threads,
            self.max_bounces,
            self.background_color,
        );
        let progressbar = if self.progressbar {
            Some(Box::new(ProgressBar::new((width*height) as u64)))
        } else {
            None
        };
        let imager = Imager::new(
            width,
            height,
            self.samples_per_pixel,
            progressbar
        );

        tokio::spawn(async move {
            let (ptx, prx) = bounded(10240);
            let (ctx, crx) = bounded(10240);
            let point_generator_handle = tokio::spawn(async move {
                point_generator.generate(ptx).await;
            });
            let sampler_handle = tokio::spawn(async move {
                sampler.sampling(&world, prx, ctx).await;
            });
            let imager_handle = tokio::spawn(async move {
                imager.collect(crx).await
            });

            point_generator_handle.await.expect("failed to join point generator thread");
            sampler_handle.await.expect("failed to join sampler thread");
            imager_handle.await.expect("failed to join imager thread")
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{camera::Camera, hittable::{sphere::Sphere, world::World}, material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal}, math::vec3::Vec3, Float};

    use super::*;

    fn dummy_world() -> Arc<World> {
        let mut world = World::new();
        world.add_material("ground", Box::new(Lambertian::new(Vec3::new(0.0, 1.0, 0.0))));
        world.add_material("center", Box::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))));
        world.add_material("left_outer", Box::new(Dielectric::new(Vec3::new_diagonal(1.0), 1.5)));
        world.add_material("left_inner", Box::new(Dielectric::new(Vec3::new_diagonal(1.0), 1.0/1.5)));
        world.add_material("right", Box::new(Metal::new(Vec3::new(0.4, 0.4, 1.0), 0.3)));

        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            world.get_material("ground").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.2),
            0.5,
            world.get_material("center").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            world.get_material("left_outer").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.4,
            world.get_material("left_inner").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            world.get_material("right").unwrap(),
        )));

        Arc::new(world)
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads=4)]
    async fn test_rendering() {
        let world = dummy_world();
        let width = 400;
        let height = 300;
        let camera = Camera::new(
            3.4,
            10.0,
            Vec3::new(-2.0, 2.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            width,
            height,
        );
        let instance = Renderer::new(
            3,
            4,
            10,
            false,
            Some(Vec3::new(0.7, 0.8, 1.0))
        );
        let image = instance.render(camera, world).await.expect("failed to join render thread");
        image.save("output/pipelined_render_test.png");
    }
}