use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::task::JoinHandle;
use flume::{bounded, Receiver, Sender};

use crate::{camera::Camera, math::vec3::Vec3, ray::Ray, utils::random::random, Float};

pub struct SamplePoint {
    pub x: usize,
    pub y: usize,
    pub ray: Ray,
}

pub struct SamplePointGenerator {
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    camera: Camera,
}

impl SamplePointGenerator {
    pub(super) fn new(
        width: usize, 
        height: usize, 
        samples_per_pixel: usize,
        camera: Camera
    ) -> Self {
        Self {
            width,
            height,
            samples_per_pixel,
            camera
         }
    }

    pub async fn generate(&self, tx: Sender<SamplePoint>) {
        for y in 0..self.height {
            for x in 0..self.width {
                for _ in 0..self.samples_per_pixel {
                    let u = (x as Float + random::<Float>()) / (self.width - 1) as Float;
                    let v = (y as Float + random::<Float>()) / (self.height - 1) as Float;
                    let ray = self.camera.get_ray(u, v);
                    tx.send_async(SamplePoint {
                        x, 
                        y, 
                        ray,
                    }).await.expect(format!("failed to send sample point").as_str());
                }
            }
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
    async fn test_dummy_generation() {
        let width = 10usize;
        let height = 5usize;
        let samples_per_pixel = 3usize;
        let camera = Camera::new(
            1.0,
            10.0,
            Vec3::zero(),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            width,
            height,
        );
        let generator = SamplePointGenerator::new(
            width,
            height,
            samples_per_pixel,
            camera,
        );

        let (tx, rx) = bounded(1024);
        let send_handle = tokio::spawn(async move {
            generator.generate(tx).await;
        });

        let recv_handle = tokio::spawn(async move {
            let mut cnt = vec![vec![0usize; height]; width];
            
            while let Ok(sample) = rx.recv_async().await {
                cnt[sample.x][sample.y] += 1;
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


