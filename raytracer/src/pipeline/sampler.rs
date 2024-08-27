use core::num;
use std::{future::Future, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use tokio::{task::{yield_now, JoinHandle}, time::{sleep, timeout}};
use flume::{bounded, Receiver, Sender};

use crate::{hittable::{world::World, Hittable}, math::vec3::Vec3, Float};

use super::{descriptor::SamplerDescriptor, dto::{SamplePoint, SampledColor, SamplerInput}};


pub struct Sampler {
    descriptor: SamplerDescriptor,
}
pub enum SamplingOutput {
    Continue(SamplerInput),
    Done(SampledColor),
}

impl Sampler {
    pub fn new(
        descriptor: SamplerDescriptor,
    ) -> Arc<Self> {
        Arc::new(Self { descriptor })
    }

    pub fn begin(
        self: Arc<Self>,
        world: Arc<World>,
        in_channel: Receiver<SamplePoint>
    ) -> (JoinHandle<()>, Receiver<SampledColor>) {
        let (tx, rx) = bounded(self.descriptor.out_buffer_size);
        let handle = tokio::spawn(async move {
            self._begin(world, in_channel, tx).await;
        });
        (handle, rx)
    }

    async fn _begin(
        &self,
        world: Arc<World>,
        in_channel: Receiver<SamplePoint>,
        out_channel: Sender<SampledColor>
    ) {
        let num_threads = self.descriptor.num_threads;
        let (tx_converted, rx_converted) = bounded(self.descriptor.in_buffer_size);
        let (tx_feedback, rx_feedback) = bounded(self.descriptor.feedback_buffer_size);

        let converter_handles: Vec<JoinHandle<()>> = (0..(num_threads+1)).map(|_| {
            let tx_converted = tx_converted.clone();
            let max_bounces = self.descriptor.max_bounces;
            let rx_in = in_channel.clone();
            let rx_feedback = rx_feedback.clone();

            tokio::spawn(async move {
                loop {
                    if !rx_feedback.is_empty() {
                        if let Ok(recv_data) = timeout(Duration::from_millis(10), rx_feedback.recv_async()).await {
                            if let Ok(sampler_input) = recv_data {
                                tx_converted.send_async(sampler_input).await.expect("failed to send converted sampler input");
                            }
                        }
                    } else if !rx_in.is_empty() {
                        if let Ok(recv_data) = timeout(Duration::from_millis(10), rx_in.recv_async()).await {
                            if let Ok(sample_point) = recv_data {
                                tx_converted.send_async(SamplerInput {
                                    x: sample_point.x,
                                    y: sample_point.y,
                                    ray: sample_point.ray,
                                    remain_bounces: max_bounces,
                                    attenuation: Vec3::new_diagonal(1.0),
                                }).await.expect("failed to send converted sampler input");
                            }
                        }
                    } else {
                        yield_now().await;
                    }
                }
            })
        }).collect();

        let sampler_handles: Vec<JoinHandle<()>> = (0..num_threads).map(|_| {
            let world = world.clone();
            let rx_in = rx_converted.clone();
            let tx_feedback = tx_feedback.clone();
            let tx_out = out_channel.clone();
            
            tokio::spawn(async move {
                loop {
                    if let Ok(r) = timeout(Duration::from_millis(10), rx_in.recv_async()).await {
                    if let Ok(sampler_input) = r {
                        match Sampler::sample(world.clone(), sampler_input) {
                            SamplingOutput::Continue(sample_point) => {
                                tx_feedback.send_async(sample_point).await
                                .expect("failed to send feedback sampler input");
                            },
                            SamplingOutput::Done(sampled_color) => {
                                tx_out.send_async(sampled_color).await
                                .expect("failed to send sampled color");
                            }
                        }
                    }}
                }
            })
        }).collect();

        tokio::spawn(async move {
            while !(in_channel.is_disconnected() && in_channel.is_empty()) ||
                  !rx_converted.is_empty() ||
                  !rx_feedback.is_empty() {
                    sleep(Duration::from_millis(100)).await;
                  }
        }).await.expect("failed to join complete checking thread");
        
        for handle in converter_handles {
            handle.abort();
        }
        for handle in sampler_handles {
            handle.abort();
        }
    }
    
    fn sample(world: Arc<World>, sample_point: SamplerInput) -> SamplingOutput {
        let x = sample_point.x;
        let y = sample_point.y;

        if sample_point.remain_bounces == 0 {
            SamplingOutput::Done(SampledColor { x, y, color: Vec3::zero() })
        }
        else if let Some(rec) = world.hit(&sample_point.ray, 0.001..Float::INFINITY) {
            if let Some((ray, attenuation)) = rec.material.scatter(&sample_point.ray, &rec) {
                SamplingOutput::Continue(SamplerInput {
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

    fn dummy_world() -> Arc<World> {
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

        Arc::new(world)
    }

    #[tokio::test]
    async fn test_dummy_sampling() {
        let num_samples = 100;
        let world = dummy_world();

        let (tx, rx) = bounded(num_samples);
        let sampler = Sampler::new(SamplerDescriptor {
            num_threads: 1,
            in_buffer_size: num_samples,
            feedback_buffer_size: num_samples,
            out_buffer_size: num_samples,
            max_bounces: 2
        });

        let (sampler_handle, color_rx) = sampler.begin(world, rx);

        let receiver_handle = tokio::spawn(async move {
            let mut received = vec![false; num_samples];
            while !color_rx.is_disconnected() {
                if let Ok(sampled_color) = color_rx.recv_async().await {
                    received[sampled_color.x] = true;
                }
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
                    x: i,
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