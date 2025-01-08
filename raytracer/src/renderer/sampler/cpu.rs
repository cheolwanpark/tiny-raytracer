use core::num;
use std::{future::Future, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use tokio::{task::{yield_now, JoinHandle}, time::{sleep, timeout}};
use flume::{bounded, Receiver, Sender};

use crate::{hittable::{bvh::BVH, world::World, Hittable}, math::vec3::Vec3, renderer::{imager::SampledColor, pointgen::SamplePoint}, Float};

use super::Sampler;

#[derive(Clone, Copy)]
pub struct CpuSampler {
    num_threads: usize,
    max_bounces: usize,
    background_color: Vec3,
}

impl CpuSampler {
    pub fn new(
        num_threads: usize,
        max_bounces: usize,
        background_color: Vec3,
    ) -> Self {
        Self { num_threads, max_bounces, background_color }
    }

    async fn sampling_subthread(
        &self, 
        world: Arc<BVH>,
        in_channel: Receiver<SamplePoint>,
        out_channel: Sender<SampledColor>,
    ) {
        while let Ok(sample_point) = in_channel.recv_async().await {
            out_channel.send_async(self.single_point_sampling(world.clone(), sample_point))
                       .await.expect("failed to send sampled color");
        }
    }

    fn single_point_sampling(&self, world: Arc<BVH>, sample_point: SamplePoint) -> SampledColor {
        let x = sample_point.x;
        let y = sample_point.y;
        let mut ray = sample_point.ray;
        let mut remain_bounces = self.max_bounces;
        let mut color = Vec3::zero();
        let mut cumulated_attenuation = Vec3::new_diagonal(Float::from(1.0));

        while remain_bounces > 0 {
            if let Some(rec) = world.hit(&ray, 0.001..Float::INFINITY) {
                let emission = rec.material.emitted().unwrap_or(Vec3::zero());
                color += cumulated_attenuation * emission;
                if let Some((new_ray, attenuation)) = rec.material.scatter(&ray, &rec) {
                    cumulated_attenuation *= attenuation;
                    ray = new_ray;
                    remain_bounces -= 1;
                } else {
                    break;
                }
            } else {
                color += cumulated_attenuation * self.background_color;
                break;
            }
        }

        SampledColor { x, y, color }
    }
}

impl Sampler for CpuSampler {
    async fn sampling(
        self, 
        world: &World,
        in_channel: Receiver<SamplePoint>,
        out_channel: Sender<SampledColor>,
    ) {
        let world = Arc::new(world.get_bvh());
        let handles: Vec<JoinHandle<()>> = (0..self.num_threads).map(|_| {
            let world = world.clone();
            let in_channel = in_channel.clone();
            let out_channel = out_channel.clone();
            tokio::spawn(async move {
                self.sampling_subthread(world, in_channel, out_channel).await;
            })
        }).collect();
        for handle in handles {
            handle.await.expect("failed to join sampling thread");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use flume::bounded;

    use super::*;
    use crate::{hittable::sphere::Sphere, material::lambertian::Lambertian, ray::Ray};

    fn dummy_world() -> Arc<World> {
        let mut world = World::new();
        world.add_material("dummy", Box::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0))));
        world.add_geometry(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            world.get_material("dummy").unwrap(),
        )));
        world.add_geometry(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.2),
            0.5,
            world.get_material("dummy").unwrap(),
        )));
        world.add_geometry(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            world.get_material("dummy").unwrap(),
        )));
        world.add_geometry(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.4,
            world.get_material("dummy").unwrap(),
        )));
        world.add_geometry(Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            world.get_material("dummy").unwrap(),
        )));

        Arc::new(world)
    }

    #[tokio::test]
    async fn test_dummy_sampling() {
        let num_samples = 100;
        let world = dummy_world();

        let (tx, rx) = bounded(num_samples);
        let (ctx, crx) = bounded(num_samples);

        let sampler = CpuSampler::new(
            1, 
            2, 
            Vec3::zero(),
        );
        let sampler_handle = tokio::spawn(async move {
            sampler.sampling(&world, rx, ctx).await;
        });

        let receiver_handle = tokio::spawn(async move {
            let mut received = vec![false; num_samples];
            while let Ok(sampled_color) = crx.recv_async().await {
                received[sampled_color.x as usize] = true;
            }
            received
        });
        
        let sender_handle = tokio::spawn(async move {
            let sample_points: Vec<SamplePoint> = (0..num_samples).map(|i| {
                let t = i as Float / num_samples as Float;
                let x = -1.0 * (1.0 - t) + 1.0 * t;
                let origin = Vec3::new(x, 0.0, 0.0);
                let ray = Ray::new(origin, Vec3::new(0.0, 0.0, -1.0));
                SamplePoint {
                    x: i as u32,
                    y: 0,
                    ray,
                }
            }).collect();
            tokio::time::sleep(Duration::from_secs(1)).await;
            for sample_point in sample_points {
                tx.send_async(sample_point).await.expect("failed to send sample point");
            }
        });

        sender_handle.await.expect("failed to join sender thread");
        sampler_handle.await.expect("failed to join sampler thread");
        let received = receiver_handle.await.expect("failed to join receive thread");
        for i in 0..received.len() {
            assert!(received[i], "failed at i={}", i)
        }
    }
}