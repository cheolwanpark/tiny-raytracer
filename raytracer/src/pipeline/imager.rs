use std::sync::Arc;

use flume::Receiver;
use indicatif::ProgressBar;
use tokio::task::JoinHandle;

use crate::{math::vec3::Vec3, utils::image::Image, Float};

use super::{descriptor::ImageDescriptor, dto::SampledColor};

pub struct Imager {
    image_descriptor: ImageDescriptor,
    progressbar: Option<Arc<ProgressBar>>,
}

impl Imager {
    pub(super) fn new(
        image_descriptor: ImageDescriptor, 
        progressbar: Option<Arc<ProgressBar>>
    ) -> Arc<Self> {
        Arc::new(Self { image_descriptor, progressbar })
    }

    pub fn begin(
        self: Arc<Self>,
        in_channel: Receiver<SampledColor>,
    ) -> JoinHandle<Image> {
        tokio::spawn(async move {
            self._begin(in_channel).await
        })
    }

    async fn _begin(
        &self,
        in_channel: Receiver<SampledColor>
    ) -> Image {
        let width = self.image_descriptor.width;
        let height = self.image_descriptor.height;
        let samples_per_pixel = self.image_descriptor.samples_per_pixel;
        let color_divisor = Float::from(1.0) / samples_per_pixel as Float;

        let mut image = Image::new_with_gamma_correction(
            width,
            height,
            2.2
        );

        let mut pixels = vec![Vec3::zero(); width*height];
        let mut num_acc_cnt = vec![0usize; width*height];

        while !in_channel.is_disconnected() {
            if let Ok(sampled_color) = in_channel.recv_async().await {
                let x = sampled_color.x;
                let y = sampled_color.y;
                let idx = y*width + x;
                pixels[idx] += sampled_color.color * color_divisor;
                num_acc_cnt[idx] += 1;
                if num_acc_cnt[idx] == samples_per_pixel {
                    image.set_pixel(x, y, pixels[idx].into());
                    if let Some(progressbar) = self.progressbar.clone() {
                        progressbar.inc(1);
                    }
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
        let image_descriptor = ImageDescriptor {
            width,
            height,
            samples_per_pixel,
        };
        let (sampler_handle, rx) = dummy_sampler(image_descriptor.clone());

        let imager = Imager::new(image_descriptor, None);
        let imager_handle = imager.begin(rx);

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

    fn dummy_sampler(image_descriptor: ImageDescriptor) -> (JoinHandle<()>, Receiver<SampledColor>) {
        let width = image_descriptor.width;
        let height = image_descriptor.height;
        let samples_per_pixel = image_descriptor.samples_per_pixel;
        let (tx, rx) = bounded(4096);
        let handle = tokio::spawn(async move {
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
        (handle, rx)
    }
}