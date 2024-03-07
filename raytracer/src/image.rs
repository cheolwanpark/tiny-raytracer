use image::{RgbImage, Rgb};

#[derive(Clone)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            data: vec![Color::new_grey(0.0); width*height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.data[y * self.width + x] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
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
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn new_grey(value: f64) -> Color {
        Color { r: value, g: value, b: value }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Rgb<u8> {
        Rgb([
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
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
                let r = i as f64 / height as f64;
                let g = j as f64 / width as f64;
                image.set_pixel(i, j, Color::new(r, g, 0.0));
            }
        }
        image.save("image-test.png");
    }
}