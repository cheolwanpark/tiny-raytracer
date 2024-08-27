use core::num;
use std::{future::Future, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use tokio::{task::JoinHandle, time::timeout};
use flume::{Receiver, Sender};

use crate::{hittable::{world::World, Hittable}, math::vec3::Vec3, Float};

use super::dto::{SamplePoint, SampledColor};


pub struct Sampler {
    world: Arc<World>,
}
pub enum SamplingOutput {
    Continue(SamplePoint),
    Done(SampledColor),
}

impl Sampler {
    pub fn new(
        world: World, 
    ) -> Arc<Self> {
        Arc::new(Self {
            world: Arc::new(world),
        })
    }

    pub fn begin(
        self: Arc<Self>,
        in_channel: Receiver<SamplePoint>,
        out_channel: Sender<SampledColor>,
        feedback_channel: Sender<SamplePoint>,
        generation_done: Arc<AtomicBool>,
        num_threads: usize
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            self._begin(
                in_channel, 
                out_channel, 
                feedback_channel,
                generation_done,
                num_threads
            ).await;
        })
    }

    async fn _begin(
        &self,
        in_channel: Receiver<SamplePoint>,
        out_channel: Sender<SampledColor>,
        feedback_channel: Sender<SamplePoint>,
        generation_done: Arc<AtomicBool>,
        num_threads: usize
    ) {
        let handles: Vec<JoinHandle<()>> = (0..num_threads).map(|i| {
            let world = self.world.clone();
            let in_rx = in_channel.clone();
            let feedback_tx = feedback_channel.clone();
            let out_tx = out_channel.clone();
            let gen_done = generation_done.clone();
            
            tokio::spawn(async move {
                while !gen_done.load(Ordering::Relaxed) || !in_rx.is_empty() {
                    if let Ok(recv_result) = timeout(Duration::from_millis(10), in_rx.recv_async()).await {
                        if let Ok(sample_point) = recv_result {
                            match Sampler::sample(world.clone(), sample_point) {
                                SamplingOutput::Continue(sample_point) => {
                                    feedback_tx.send_async(sample_point).await
                                    .expect(format!("failed to send feedback sample point on thread{}", i).as_str());
                                },
                                SamplingOutput::Done(sampled_color) => {
                                    out_tx.send_async(sampled_color).await
                                    .expect(format!("failed to send sampled color on thread{}", i).as_str());
                                }
                            }
                        }
                    }
                }
            })
        }).collect();

        for handle in handles {
            handle.await.expect("failed to join thread")
        }
    }
    
    fn sample(world: Arc<World>, sample_point: SamplePoint) -> SamplingOutput {
        let x = sample_point.x;
        let y = sample_point.y;

        if sample_point.remain_bounces == 0 {
            SamplingOutput::Done(SampledColor { x, y, color: Vec3::zero() })
        }
        else if let Some(rec) = world.hit(&sample_point.ray, 0.001..Float::INFINITY) {
            if let Some((ray, attenuation)) = rec.material.scatter(&sample_point.ray, &rec) {
                SamplingOutput::Continue(SamplePoint {
                    x, y,
                    ray,
                    remain_bounces: sample_point.remain_bounces - 1,
                    attenuation: sample_point.attenuation * attenuation
                })
            } else {
                SamplingOutput::Done(SampledColor { x, y, color: Vec3::zero() })
            }
        } else {
            let direction = sample_point.ray.direction();
            let a = 0.5 * (direction.y + 1.0);
            let attenuation = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
            SamplingOutput::Done(SampledColor {
                x, y,
                color: sample_point.attenuation * attenuation
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use flume::bounded;

    use super::*;
    use crate::{hittable::sphere::Sphere, material::lambertian::Lambertian, ray::Ray};

    fn dummy_world() -> World {
        let mut world = World::new();
        world.add_material("dummy", Box::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0))));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.2),
            0.5,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.4,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            world.get_material("dummy").unwrap(),
        )));
        world
    }

    #[tokio::test]
    async fn test_dummy_sampling() {
        let num_samples = 100;
        let world = dummy_world();

        let sample_points: Vec<SamplePoint> = (0..num_samples).map(|i| {
            let t = i as Float / num_samples as Float;
            let x = -1.0 * (1.0 - t) + 1.0 * t;
            let origin = Vec3::new(x, 0.0, 0.0);
            let ray = Ray::new(origin, Vec3::new(0.0, 0.0, -1.0));
            SamplePoint {
                x: i,
                y: 0,
                ray,
                remain_bounces: 3,
                attenuation: Vec3::new_diagonal(1.0)
            }
        }).collect();

        let (sample_point_tx, sample_point_rx) = bounded(num_samples);
        let (color_tx, color_rx) = bounded(num_samples);
        let gen_done = Arc::new(AtomicBool::new(false));

        let sampler = Sampler::new(world);
        let sampler_thread_handle = sampler.begin(sample_point_rx, color_tx, sample_point_tx.clone(), gen_done.clone(), 4);

        let receive_thread_handle = tokio::spawn(async move {
            let mut received = vec![false; num_samples];
            while !color_rx.is_disconnected() {
                if let Ok(sampled_color) = color_rx.recv_async().await {
                    received[sampled_color.x] = true;
                }
            }
            received
        });
        
        tokio::time::sleep(Duration::from_secs(1)).await;
        for sample_point in sample_points {
            sample_point_tx.send_async(sample_point).await.expect("failed to send sample point");
        }
        drop(sample_point_tx);
        gen_done.store(true, Ordering::Relaxed);

        sampler_thread_handle.await.expect("failed to join sampler thread");
        let received = receive_thread_handle.await.expect("failed to join receive thread");
        for v in received {
            assert!(v);
        }
    }
}