use std::{collections::{BTreeMap, VecDeque}, mem::transmute, path::PathBuf, slice};

use flume::{bounded, Receiver, Sender};
use metal::{Buffer, CommandBufferRef, CommandQueue, Device, Function, IntersectionFunctionTable, Library, MTLResourceOptions, MTLResourceUsage, MTLSize, NSRange, NSUInteger, Resource};
use tokio::spawn;

use crate::{hittable::{world::World, HitRecord}, material::metal::Metal, math::vec3::Vec3, ray::Ray, renderer::{imager::SampledColor, pointgen::SamplePoint, sampler::Sampler}};

use super::{accelstructure::MetalAccelerationStructure, global_resource_option, pipeline::MetalRaytracingPipeline};

const NUM_OF_RAYS_PER_GROUP: usize = 1024*10;

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
        let pipeline = MetalRaytracingPipeline::new(&device, &library);
        Self { 
            max_bounces, 
            background_color, 
            device, 
            library, 
            comm_queue,
            pipeline,
        }
    }

    fn add_command(
        &self,
        in_samples: &mut VecDeque<SamplePoint>,
        accel_structure: &MetalAccelerationStructure,
        intersection_function_table: &IntersectionFunctionTable,
    ) -> (&CommandBufferRef, Buffer) {
        let group_size = NUM_OF_RAYS_PER_GROUP.min(in_samples.len());
        let command_buffer = self.comm_queue.new_command_buffer();
        let mut samples = Vec::with_capacity(NUM_OF_RAYS_PER_GROUP);
        for _ in 0..group_size {
            samples.push(in_samples.pop_front().unwrap());
        }
        let colors = self.device.new_buffer(
            (samples.len() * size_of::<SampledColor>()) as NSUInteger,
            MTLResourceOptions::StorageModeShared
        );
        let samples = self.device.new_buffer_with_data(
            unsafe{ transmute(samples.as_ptr()) },
            (samples.len() * size_of::<SamplePoint>()) as NSUInteger,
            MTLResourceOptions::StorageModeManaged);
        samples.did_modify_range(
            NSRange::new(0, samples.length() as NSUInteger)
        );

        let encoder = command_buffer.new_compute_command_encoder();
        encoder.set_buffer(0, Some(&samples), 0);
        encoder.set_buffer(1, Some(&colors), 0);
        encoder.set_buffer(2, Some(&accel_structure.instance_buffer()), 0);
        encoder.set_acceleration_structure(3, Some(&accel_structure.instance()));
        encoder.set_intersection_function_table(4, Some(&intersection_function_table));
        for primitive_structure in accel_structure.primitive_structures() {
            let res: Resource = From::from(primitive_structure.clone());
            encoder.use_resource(&res, MTLResourceUsage::Read);
        }
        encoder.set_compute_pipeline_state(&self.pipeline.state());

        let num_threads = self.pipeline.state().thread_execution_width().min(group_size as u64);
        let thread_group_count = (group_size as u64 + num_threads - 1) / num_threads;
        encoder.dispatch_thread_groups(
            MTLSize::new(thread_group_count, 1, 1), 
            MTLSize::new(num_threads, 1, 1),
        );
        encoder.end_encoding();
        command_buffer.commit();
        (command_buffer, colors)
    }
}

impl Sampler for MetalSampler {
    async fn sampling(
            self, 
            world: &World,
            in_channel: Receiver<SamplePoint>,
            out_channel: Sender<SampledColor>,
        ) {
        let accel_structure = MetalAccelerationStructure::new(
            world, 
            &self.device, 
            &self.comm_queue
        );
        let intersection_function_table = self.pipeline.setup_intersection_function_table(
            &self.device,
            world
        );
        let mut samples =  VecDeque::new();
        let mut groups = Vec::new();
        while let Ok(sample_point) = in_channel.recv_async().await {
            samples.push_back(sample_point);
            while samples.len() >= NUM_OF_RAYS_PER_GROUP {
                groups.push(self.add_command(&mut samples, &accel_structure, &intersection_function_table));
            }
        }
        if !samples.is_empty() {
            groups.push(self.add_command(&mut samples, &accel_structure, &intersection_function_table));
        }

        for (command, colors) in groups {
            command.wait_until_completed();
            let colors = unsafe { 
                slice::from_raw_parts(
                    colors.contents() as *const SampledColor, 
                    colors.length() as usize / size_of::<SampledColor>()
                ) 
            };
            for color in colors {
                out_channel.send_async(color.clone()).await.expect("failed to send sampled color");
            }
        }
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
    use std::{sync::Arc, time::Duration};

    use flume::bounded;

    use crate::{hittable::{sphere::Sphere, world::World}, material::{lambertian::Lambertian, metal::Metal}, math::vec3::Vec3, ray::Ray, renderer::{pointgen::SamplePoint, sampler::Sampler}, Float};

    use super::MetalSampler;

    #[test]
    fn test_construction() {
        MetalSampler::new(10, Vec3::zero());
    }

    fn dummy_world() -> World {
        let mut world = World::new();
        world.add_material("dummy", Box::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0))));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.2),
            0.5,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.4,
            world.get_material("dummy").unwrap(),
        )));
        world.add_hittable(Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            world.get_material("dummy").unwrap(),
        )));

        world
    }

    #[tokio::test]
    async fn test_dummy_sampling() {
        let num_samples = 100;
        let world = dummy_world();

        let (tx, rx) = bounded(num_samples);
        let (ctx, crx) = bounded(num_samples);

        let sampler = MetalSampler::new(
            1,
            Vec3::zero(),
        );
        let sampler_handle = tokio::spawn(async move {
            sampler.sampling(&world, rx, ctx).await;
        });

        let receiver_handle = tokio::spawn(async move {
            let mut received = vec![false; num_samples];
            while let Ok(sampled_color) = crx.recv_async().await {
                received[sampled_color.x as usize] = true;
            }
            received
        });
        
        let sender_handle = tokio::spawn(async move {
            let sample_points: Vec<SamplePoint> = (0..num_samples).map(|i| {
                let t = i as Float / num_samples as Float;
                let x = -1.0 * (1.0 - t) + 1.0 * t;
                let origin = Vec3::new(x, 0.0, 0.0);
                let ray = Ray::new(origin, Vec3::new(0.0, 0.0, -1.0));
                SamplePoint {
                    x: i as u32,
                    y: 0,
                    ray,
                }
            }).collect();
            tokio::time::sleep(Duration::from_secs(1)).await;
            for sample_point in sample_points {
                tx.send_async(sample_point).await.expect("failed to send sample point");
            }
        });

        sender_handle.await.expect("failed to join sender thread");
        sampler_handle.await.expect("failed to join sampler thread");
        let received = receiver_handle.await.expect("failed to join receive thread");
        for i in 0..received.len() {
            assert!(received[i], "failed at i={}", i)
        }
    }
}