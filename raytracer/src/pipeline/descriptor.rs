use crate::camera::Camera;

pub struct ImageDescriptor {
    pub width: usize,
    pub height: usize,
}

pub struct SamplePointGeneratorDescriptor {
    pub image: ImageDescriptor,
    pub samples_per_pixel: usize,
    pub max_bounces: u32,
    pub camera: Camera,
}