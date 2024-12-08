use std::sync::Arc;

use flume::Receiver;
use indicatif::ProgressBar;
use tokio::task::JoinHandle;

use crate::{math::vec3::Vec3, utils::image::Image, Float};

pub struct SampledColor {
    pub x: usize,
    pub y: usize,
    pub color: Vec3,
}

pub struct Imager {
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    progressbar: Option<Box<ProgressBar>>,
}

impl Imager {
    pub(super) fn new(
        width: usize,
        height: usize,
        samples_per_pixel: usize,
        progressbar: Option<Box<ProgressBar>>
    ) -> Self {
        Self { width, height, samples_per_pixel, progressbar }
    }

    pub async fn collect(&self, in_channel: Receiver<SampledColor>) -> Image {
        let color_multiplier = Float::from(1.0) / self.samples_per_pixel as Float;

        let mut image = Image::new_with_gamma_correction(
            self.width,
            self.height,
            2.2
        );

        let mut pixels = vec![Vec3::zero(); self.width*self.height];
        let mut num_acc_cnt = vec![0usize; self.width*self.height];

        while let Ok(sampled_color) = in_channel.recv_async().await {
            let x = sampled_color.x;
            let y = sampled_color.y;
            let idx = y*self.width + x;
            pixels[idx] += sampled_color.color * color_multiplier;
            num_acc_cnt[idx] += 1;
            if num_acc_cnt[idx] == self.samples_per_pixel {
                image.set_pixel(x, y, pixels[idx].into());
                if let Some(progressbar) = self.progressbar.clone() {
                    progressbar.inc(1);
                }
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use tokio;
    use flume::bounded;

    use crate::utils::image::Color;

    use super::*;

    #[tokio::test]
    async fn test_dummy_imaging() {
        let width = 100;
        let height = 100;
        let samples_per_pixel = 3;

        let (tx, rx) = bounded(4096);
        let sampler_handle = tokio::spawn(async move {
            for y in 0..height {
                for x in 0..width {
                    for _ in 0..samples_per_pixel {
                        let color_x = ((x+1)*(y+1)) as Float / (width*height) as Float;
                        tx.send_async(SampledColor {
                            x,
                            y,
                            color: Vec3::new(color_x, 0.0, 0.0),
                        }).await.expect("failed to send color data");
                    }
                }
            }
        });

        let imager = Imager::new(width, height, samples_per_pixel, None);
        let imager_handle = tokio::spawn(async move {
            imager.collect(rx).await
        });

        sampler_handle.await.expect("failed to join sampler thread");
        let image = imager_handle.await.expect("failed to join imager thread");
        for y in 0..height {
            for x in 0..width {
                let correct_val = ((x+1)*(y+1)) as Float / (width*height) as Float;
                let correct_val = Color::new(correct_val, 0.0, 0.0).gamma_correction(2.2).r;
                assert!((image.get_pixel(x, y).r - correct_val) < 1e-5);
            }
        }
    }
}