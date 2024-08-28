use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::task::JoinHandle;
use flume::{bounded, Receiver, Sender};

use crate::{camera::Camera, math::vec3::Vec3, utils::random::random_float, Float};

use super::{descriptor::SamplePointGeneratorDescriptor, dto::SamplePoint};

pub struct SamplePointGenerator {
    descriptor: SamplePointGeneratorDescriptor,
}

impl SamplePointGenerator {
    pub(super) fn new(descriptor: SamplePointGeneratorDescriptor) -> Arc<Self> {
        Arc::new(Self { descriptor })
    }

    pub fn begin(self: Arc<Self>) -> (JoinHandle<()>, Receiver<SamplePoint>) {
        let (tx, rx) = bounded(self.descriptor.buffer_size);
        let handle = tokio::spawn(async move {
            self._begin(tx).await;
        });
        (handle, rx)
    }

    async fn _begin(&self, out_channel: Sender<SamplePoint>) {
        let num_threads = self.descriptor.num_threads;
        let width = self.descriptor.image.width;
        let height = self.descriptor.image.height;
        let samples_per_pixel = self.descriptor.image.samples_per_pixel;
        let cols_per_thread = height / num_threads;
        
        let handles: Vec<JoinHandle<()>> = (0..num_threads).map(|i| {
            let camera = self.descriptor.camera.clone();
            let sender = out_channel.clone();

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
                            }).await.expect(format!("failed to send sample point on thread#{}", i).as_str());
                        }
                    }
                }
            })
        }).collect();

        for handle in handles {
            handle.await.expect("failed to join thread");
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio;
    use flume::bounded;

    use super::*;
    use crate::{math::vec3::Vec3, pipeline::descriptor::ImageDescriptor, Float};

    #[tokio::test]
    async fn test_dummy_generation() {
        let width = 100usize;
        let height = 60usize;
        let samples_per_pixel = 3usize;
        let generator = SamplePointGenerator::new(SamplePointGeneratorDescriptor {
            num_threads: 2,
            buffer_size: 1024,
            image: ImageDescriptor { 
                width,
                height,
                samples_per_pixel,
            },
            camera: Camera::new(
                1.0,
                10.0,
                Vec3::zero(),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                width as Float / height as Float,
            )
        });

        let (send_handle, rx) = generator.begin();

        
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


