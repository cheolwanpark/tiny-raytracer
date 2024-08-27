use crate::{camera::Camera, hittable::world::World};

#[derive(Clone)]
pub struct ImageDescriptor {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
}

#[derive(Clone)]
pub struct SamplePointGeneratorDescriptor {
    pub image: ImageDescriptor,
    pub max_bounces: u32,
    pub camera: Camera,
}

pub struct InstanceDescriptor {
    pub num_point_generator_threads: usize,
    pub num_sampler_threads: usize,
    pub sample_point_buffer_size: usize,
    pub sampled_color_buffer_size: usize,
    pub point_generator_descriptor: SamplePointGeneratorDescriptor,
    pub world: World,
    pub progressbar: bool,
}