use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::task::JoinHandle;
use flume::Sender;

use crate::{camera::Camera, math::vec3::Vec3, utils::{image::Image, random::random_float}, Float};

use super::{descriptor::SamplePointGeneratorDescriptor, dto::SamplePoint};

pub struct SamplePointGenerator {
    descriptor: SamplePointGeneratorDescriptor,
}

impl SamplePointGenerator {
    pub(super) fn new(descriptor: SamplePointGeneratorDescriptor) -> Arc<Self> {
        Arc::new(Self { descriptor })
    }

    fn begin(self: Arc<Self>, sender: Sender<SamplePoint>, num_threads: usize) -> (JoinHandle<()>, Arc<AtomicBool>) {
        let done = Arc::new(AtomicBool::new(false));
        let done_ret = done.clone();
        let handle = tokio::spawn(async move {
            self._begin(sender, num_threads).await;
            done.store(true, Ordering::Relaxed);
        });
        (handle, done_ret)
    }

    async fn _begin(&self, sender: Sender<SamplePoint>, num_threads: usize) {
        let width = self.descriptor.width;
        let height = self.descriptor.height;
        let samples_per_pixel = self.descriptor.samples_per_pixel;
        let cols_per_thread = height / num_threads;
        
        let handles: Vec<JoinHandle<()>> = (0..num_threads).map(|i| {
            let camera = self.descriptor.camera.clone();
            let sender = sender.clone();
            let max_bounces = self.descriptor.max_bounces;

            tokio::spawn(async move {
                let cols_beg = cols_per_thread * i;
                let cols_end = if i == num_threads-1 {
                    height
                } else {
                    cols_per_thread * (i+1)
                };

                for y in cols_beg..cols_end {
                    for x in 0..width {
                        for _ in 0..samples_per_pixel {
                            let u = (x as Float + random_float()) / (width - 1) as Float;
                            let v = (y as Float + random_float()) / (height - 1) as Float;
                            let ray = camera.get_ray(u, v);
                            sender.send_async(SamplePoint {
                                x, 
                                y, 
                                ray,
                                remain_bounces: max_bounces,
                                attenuation: Vec3::new_diagonal(1.0),
                            }).await.expect(format!("failed to send sample point on thread#{}", i).as_str());
                        }
                    }
                }
            })
        }).collect();

        for handle in handles {
            handle.await.expect("failed to join thread")
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio;
    use flume::bounded;

    use super::*;
    use crate::{math::vec3::Vec3, Float};

    #[tokio::test]
    pub async fn test_dummy_generation() {
        let width = 100usize;
        let height = 60usize;
        let samples_per_pixel = 3usize;
        let generator = SamplePointGenerator::new(SamplePointGeneratorDescriptor {
            width,
            height,
            samples_per_pixel,
            max_bounces: 5,
            camera: Camera::new(
                1.0,
                Vec3::zero(),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                width as Float / height as Float,
            )
        });

        let (tx, rx) = bounded(128);
        let (send_handle, _) = generator.begin(tx, 2);

        
        let recv_handle = tokio::spawn(async move {
            let mut cnt = vec![vec![0usize; height]; width];
            while !rx.is_disconnected() {
                if let Ok(sample) = rx.recv_async().await {
                    cnt[sample.x][sample.y] += 1;
                }
            }
            cnt
        });
        send_handle.await.expect("failed to join send thread");
        let cnt = recv_handle.await.expect("failed to join receive thread");

        for x in 0..width {
            for y in 0..height {
                assert_eq!(cnt[x][y], samples_per_pixel);
            }
        }
    }
}


