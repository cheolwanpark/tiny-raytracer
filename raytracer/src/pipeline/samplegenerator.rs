use std::sync::Arc;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::{camera::Camera, utils::{image::Image, random::random_float}, Float};

use super::{descriptor::ImageDescriptor, dto::Sample};

pub struct SampleGenerator {
    image_descriptor: ImageDescriptor,
    camera: Arc<Camera>,
}

impl SampleGenerator {
    pub(super) fn new(image_descriptor: ImageDescriptor, camera: Camera) -> Arc<Self> {
        Arc::new(Self { 
            image_descriptor, 
            camera: Arc::new(camera), 
        })
    }

    pub async fn begin(self: Arc<Self>, sender: Sender<Sample>, num_threads: usize) -> JoinHandle<()> {
        tokio::spawn(async move {
            self._begin(sender, num_threads).await;
        })
    }

    async fn _begin(&self, sender: Sender<Sample>, num_threads: usize) {
        let width = self.image_descriptor.width;
        let height = self.image_descriptor.height;
        let samples_per_pixel = self.image_descriptor.samples_per_pixel;
        let cols_per_thread = height / num_threads;
        
        let handles: Vec<JoinHandle<()>> = (0..num_threads).map(|i| {
            let camera = self.camera.clone();
            let sender = sender.clone();

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

                            sender.send(Sample {
                                x, y, ray
                            }).await
                            .expect(format!("Failed to send sample on thread#{}", i).as_str());
                        }
                    }
                }
            })
        }).collect();

        for handle in handles {
            handle.await.expect("Failed to join thread#{}")
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::{sync::mpsc::channel, task::JoinHandle};

    use super::*;
    use crate::{math::vec3::Vec3, Float};

    #[tokio::test]
    pub async fn test_multithreaded_generation() {
        let width = 640usize;
        let height = 480usize;
        let samples_per_pixel = 10usize;
        let image_descriptor = ImageDescriptor {
            width,
            height,
            samples_per_pixel,
        };
        let camera = Camera::new(
            1.0,
            Vec3::zero(),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            width as Float / height as Float,
        );
        let generator = SampleGenerator::new(image_descriptor, camera);

        let (tx, mut rx) = channel(128);
        let send_handles = generator.begin(tx, 3);

        let recv_handle = tokio::spawn(async move {
            let mut cnt = vec![vec![0usize; height]; width];
            let mut buffer = Vec::new();
            while !rx.is_closed() {
                buffer.clear();
                rx.recv_many(&mut buffer, 16).await;
                for sample in &buffer {
                    cnt[sample.x][sample.y] += 1;
                }
            }
            cnt
        });
        send_handles.await;
        let cnt = recv_handle.await.expect("Failed to join receive thread");

        for x in 0..width {
            for y in 0..height {
                assert_eq!(cnt[x][y], samples_per_pixel);
            }
        }
    }
}