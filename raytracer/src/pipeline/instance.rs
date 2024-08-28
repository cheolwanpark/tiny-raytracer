use std::sync::Arc;

use flume::bounded;
use indicatif::ProgressBar;
use tokio::{self, task::JoinHandle};

use crate::{hittable::world::World, utils::image::Image};

use super::{descriptor::InstanceDescriptor, imager::Imager, pointgen::SamplePointGenerator, sampler::Sampler};

pub struct Instance {
    point_generator: Arc<SamplePointGenerator>,
    sampler: Arc<Sampler>,
    imager: Arc<Imager>,
}

impl Instance {
    pub fn new(descriptor: InstanceDescriptor) -> Arc<Self> {
        let image_desciptor = descriptor.point_generator_descriptor.image.clone();
        let point_generator = SamplePointGenerator::new(
            descriptor.point_generator_descriptor
        );
        let sampler = Sampler::new(descriptor.sampler_descriptor);
        let progressbar = if descriptor.progressbar {
            Some(Arc::new(ProgressBar::new((image_desciptor.width * image_desciptor.height) as u64)))
        } else {
            None
        };
        let imager = Imager::new(image_desciptor, progressbar);

        Arc::new(Self {
            point_generator,
            sampler,
            imager
        })
    }

    pub fn begin(self: Arc<Self>, world: Arc<World>) -> JoinHandle<Image> {
        tokio::spawn(async move {
            self._begin(world).await
        })
    }

    async fn _begin(&self, world: Arc<World>) -> Image {
        let (point_generator_handle, rx_sample_point) = self.point_generator.clone().begin();
        let (sampler_handle, rx_color) = self.sampler.clone().begin(world, rx_sample_point);
        let imager_handle = self.imager.clone().begin(rx_color);

        point_generator_handle.await.expect("failed to join point generator thread");
        sampler_handle.await.expect("failed to join sampler thread");
        imager_handle.await.expect("failed to join imager thread")
    }
}

#[cfg(test)]
mod tests {
    use crate::{camera::Camera, hittable::{sphere::Sphere, world::World}, material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal}, math::vec3::Vec3, pipeline::descriptor::{ImageDescriptor, SamplePointGeneratorDescriptor, SamplerDescriptor}, Float};

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
        let instance = Instance::new(InstanceDescriptor {
            point_generator_descriptor: SamplePointGeneratorDescriptor {
                num_threads: 2,
                buffer_size: 2048,
                image: ImageDescriptor {
                    width: width,
                    height: height,
                    samples_per_pixel: 3,
                },
                camera: Camera::new(
                    3.4,
                    10.0,
                    Vec3::new(-2.0, 2.0, 1.0),
                    Vec3::new(0.0, 0.0, -1.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    20.0,
                    width as Float / height as Float,
                )
            },
            sampler_descriptor: SamplerDescriptor {
                num_threads: 4,
                in_buffer_size: 4096,
                feedback_buffer_size: 2048,
                out_buffer_size: 10240,
                max_bounces: 10,
            },
            progressbar: false,
        });
        let image = instance.begin(world).await.expect("failed to join render thread");
        image.save("output/pipelined_render_test.png");
    }
}