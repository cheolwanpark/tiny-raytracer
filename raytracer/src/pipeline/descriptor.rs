use crate::camera::Camera;

pub struct SamplePointGeneratorDescriptor {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: u32,
    pub camera: Camera,
}