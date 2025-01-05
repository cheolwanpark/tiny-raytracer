use std::{collections::BTreeMap, path::PathBuf};

use flume::{Receiver, Sender};
use metal::{CommandQueue, Device, Function, Library};

use crate::{hittable::world::World, math::vec3::Vec3, renderer::{imager::SampledColor, pointgen::SamplePoint, sampler::Sampler}};

use super::pipeline::MetalRaytracingPipeline;

struct MetalSampler {
    max_bounces: usize,
    background_color: Vec3,
    device: Device,
    library: Library,
    comm_queue: CommandQueue,
    pipeline: MetalRaytracingPipeline,
}

impl MetalSampler {
    pub fn new(
        max_bounces: usize, 
        background_color: Vec3
    ) -> Self {
        let device = find_raytracing_supporting_device();
        let lib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/renderer/sampler/metal/shader.metallib");
        let library = device.new_library_with_file(lib_path).expect("failed to load meatallib");
        let comm_queue = device.new_command_queue();
        let pipeline = MetalRaytracingPipeline::setup(&device, &library);
        Self { 
            max_bounces, 
            background_color, 
            device, 
            library, 
            comm_queue,
            pipeline,
        }
    }
}

impl Sampler for MetalSampler {
    async fn sampling(
            self, 
            world: &World,
            in_channel: Receiver<SamplePoint>,
            out_channel: Sender<SampledColor>,
        ) {
        // refer https://github.com/gfx-rs/metal-rs/blob/ef768ff9d742ae6a0f4e83ddc8031264e7d460c4/examples/raytracing/renderer.rs#L361
        unimplemented!()
    }
}

fn find_raytracing_supporting_device() -> Device {
    for device in Device::all() {
        if !device.supports_raytracing() {
            continue;
        }
        if device.is_low_power() {
            continue;
        }
        return device;
    }

    panic!("No device in this machine supports raytracing!")
}


#[cfg(test)]
mod tests {
    use crate::math::vec3::Vec3;

    use super::MetalSampler;

    #[test]
    fn test_construction() {
        MetalSampler::new(10, Vec3::zero());
    }
}