use std::{mem::transmute, os::raw::c_void};

use metal::{AccelerationStructure, AccelerationStructureDescriptor, Array, Buffer, CommandQueue, Device, InstanceAccelerationStructureDescriptor, MTLAccelerationStructureInstanceDescriptor, MTLAccelerationStructureInstanceOptions, MTLResourceOptions, NSRange, NSUInteger, PrimitiveAccelerationStructureDescriptor};

use crate::hittable::world::World;

use super::global_resource_option;

pub struct MetalAccelerationStructure {
    instance: AccelerationStructure,
    primitive_structures: Vec<AccelerationStructure>,
    instance_descriptors: Vec<MTLAccelerationStructureInstanceDescriptor>,
    instance_buffer: Buffer,
}

impl MetalAccelerationStructure {
    pub fn new(
        world: &World, 
        device: &Device, 
        queue: &CommandQueue
    ) -> Self {
        let metal_geometries = world.get_metal_geometries(device.clone());
        let primitive_structures: Vec<AccelerationStructure> = metal_geometries
             .iter()
             .enumerate()
             .map(|(i, geometry)| {
                let geom_descriptor = geometry.get_geometry_descriptor();
                geom_descriptor.set_intersection_function_table_offset(i as NSUInteger);
                let geom_descriptors = Array::from_owned_slice(&[geom_descriptor]);
                let accel_descriptor = PrimitiveAccelerationStructureDescriptor::descriptor();
                accel_descriptor.set_geometry_descriptors(geom_descriptors);
                let accel_descriptor = AccelerationStructureDescriptor::from(accel_descriptor);
                new_acceleration_structure(device, queue, &accel_descriptor)
             }).collect();

        let instance_descriptors: Vec<MTLAccelerationStructureInstanceDescriptor> = metal_geometries
             .iter()
             .enumerate()
             .map(|(i, geometry)| {
                let mut descriptor = MTLAccelerationStructureInstanceDescriptor::default();
                descriptor.acceleration_structure_index = i as u32;
                descriptor.options = if geometry.get_intersection_function_name().is_none() {
                    MTLAccelerationStructureInstanceOptions::Opaque
                } else {
                    MTLAccelerationStructureInstanceOptions::None
                };
                descriptor.intersection_function_table_offset = 0;
                descriptor.mask = geometry.get_mask();
                descriptor
             }).collect();
        let instance_buffer = device.new_buffer_with_data(
            instance_descriptors.as_ptr() as *const c_void,
            (size_of::<MTLAccelerationStructureInstanceDescriptor>()
                * metal_geometries.len()) as NSUInteger,
            global_resource_option(),
        );
        instance_buffer.set_label("instance buffer");
        instance_buffer.did_modify_range(NSRange::new(0, instance_buffer.length()));
        
        let accel_descriptor = InstanceAccelerationStructureDescriptor::descriptor();
        accel_descriptor.set_instanced_acceleration_structures(
            &Array::from_owned_slice(&primitive_structures)
        );
        accel_descriptor.set_instance_count(metal_geometries.len() as NSUInteger);
        accel_descriptor.set_instance_descriptor_buffer(&instance_buffer);
        let accel_descriptor: AccelerationStructureDescriptor = accel_descriptor.into();
        let instance = new_acceleration_structure(&device, queue, &accel_descriptor);

        Self {
            instance,
            primitive_structures,
            instance_descriptors,
            instance_buffer
        }
    }

    pub fn instance(&self) -> &AccelerationStructure {
        &self.instance
    }

    pub fn primitive_structures(&self) -> &Vec<AccelerationStructure> {
        &self.primitive_structures
    }

    pub fn instance_buffer(&self) -> &Buffer {
        &self.instance_buffer
    }
}

fn new_acceleration_structure(
    device: &Device,
    queue: &CommandQueue,
    descriptor: &AccelerationStructureDescriptor
) -> AccelerationStructure {
    let size = device.acceleration_structure_sizes_with_descriptor(descriptor);
    let acceleration_structure = device.new_acceleration_structure_with_size(size.acceleration_structure_size);
    let scratch_buffer = device.new_buffer(
        size.build_scratch_buffer_size,
        MTLResourceOptions::StorageModePrivate,
    );
    let command_buffer = queue.new_command_buffer();
    let command_encoder = command_buffer.new_acceleration_structure_command_encoder();
    let compacted_size_buffer = device.new_buffer(
        size_of::<u32>() as NSUInteger,
        MTLResourceOptions::StorageModeShared,
    );
    command_encoder.build_acceleration_structure(
        &acceleration_structure, 
        &descriptor, 
        &scratch_buffer, 
        0,
    );
    command_encoder.write_compacted_acceleration_structure_size(
        &acceleration_structure, 
        &compacted_size_buffer, 
        0,
    );
    command_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    let compacted_size: *const u32 = unsafe { transmute(compacted_size_buffer.contents()) };
    let compacted_size = unsafe { *compacted_size } as NSUInteger;
    let compacted_acceleration_structure =
        device.new_acceleration_structure_with_size(compacted_size);
    let command_buffer = queue.new_command_buffer();
    let command_encoder = command_buffer.new_acceleration_structure_command_encoder();
    command_encoder.copy_and_compact_acceleration_structure(
        &acceleration_structure,
        &compacted_acceleration_structure,
    );
    command_encoder.end_encoding();
    command_buffer.commit();
    compacted_acceleration_structure
}