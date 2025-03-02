use crate::Float;
use image::{RgbImage, Rgb};

#[derive(Clone)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
    gamma: Option<Float>,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            data: vec![Color::new_grey(0.0); width*height],
            gamma: None,
        }
    }

    pub fn new_with_gamma_correction(width: usize, height: usize, gamma: Float) -> Image {
        Image {
            width,
            height,
            data: vec![Color::new_grey(0.0); width*height],
            gamma: Some(gamma)
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.data[y * self.width + x] = if let Some(gamma) = self.gamma {
            color.gamma_correction(gamma)
        } else {
            color
        };
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn aspect_ratio(&self) -> Float {
        self.width as Float / self.height as Float
    }

    pub fn save(self, filename: &str) {
        let img = RgbImage::from(self);
        img.save(filename).unwrap();
    }
}

impl From<Image> for RgbImage {
    fn from(image: Image) -> RgbImage {
        let mut img = RgbImage::new(image.width as u32, image.height as u32);
        for (i, color) in image.data.iter().enumerate() {
            img.put_pixel((i % image.width) as u32, (i / image.width) as u32, Rgb::from(*color));
        }
        img
    }

}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Color {
        Color { r, g, b }
    }

    pub fn new_grey(value: Float) -> Color {
        Color { r: value, g: value, b: value }
    }

    pub fn gamma_correction(&self, gamma: Float) -> Color {
        Color {
            r: self.r.powf(1.0 / gamma),
            g: self.g.powf(1.0 / gamma),
            b: self.b.powf(1.0 / gamma),
        }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Rgb<u8> {
        const INTENSITY_MIN: Float = 0.000;
        const INTENSITY_MAX: Float = 0.999;
        Rgb([
            (color.r.clamp(INTENSITY_MIN, INTENSITY_MAX) * 255.0) as u8,
            (color.g.clamp(INTENSITY_MIN, INTENSITY_MAX) * 255.0) as u8,
            (color.b.clamp(INTENSITY_MIN, INTENSITY_MAX) * 255.0) as u8,
        ])
    }
}

mod test {
    #[test]
    #[ignore]
    fn test_image_generation() {
        use super::*;
        let mut image = Image::new(256, 256);
        let (width, height) = image.size();
        for j in 0..height {
            for i in 0..width {
                let r = i as Float / height as Float;
                let g = j as Float / width as Float;
                image.set_pixel(i, j, Color::new(r, g, 0.0));
            }
        }
        image.save("output/image-test.png");
    }
}