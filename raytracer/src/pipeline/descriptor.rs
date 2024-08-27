use crate::{camera::Camera, hittable::world::World};

#[derive(Clone)]
pub struct ImageDescriptor {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
}

#[derive(Clone)]
pub struct SamplePointGeneratorDescriptor {
    pub num_threads: usize,
    pub buffer_size: usize,
    pub image: ImageDescriptor,
    pub camera: Camera,
}

pub struct SamplerDescriptor {
    pub num_threads: usize,
    pub in_buffer_size: usize,
    pub feedback_buffer_size: usize,
    pub out_buffer_size: usize,
    pub max_bounces: usize,
}

pub struct InstanceDescriptor {
    pub point_generator_descriptor: SamplePointGeneratorDescriptor,
    pub sampler_descriptor: SamplerDescriptor,
    pub progressbar: bool,
}