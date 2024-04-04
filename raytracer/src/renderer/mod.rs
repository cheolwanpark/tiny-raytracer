use crate::{camera::Camera, hittable::Hittable, image::Image, math::vec3::Vec3, ray::Ray};

pub struct ImageOptions {
    pub width: usize,
    pub height: usize,
}

impl ImageOptions {
    pub fn new(width: usize, height: usize) -> ImageOptions {
        ImageOptions { width, height }
    }
}

pub trait Renderer {
    fn render(&self, camera: Camera, world: Box<dyn Hittable>, image_options: ImageOptions) -> Image;
}

pub trait ColorSampler {
    fn sample(&self, ray: Ray, world: &Box<dyn Hittable>, depth: u32) -> Vec3;
}

pub mod bruteforce;
pub mod colorsampler;