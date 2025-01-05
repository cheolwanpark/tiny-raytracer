use metal::MTLResourceOptions;

pub mod geometry;
mod sampler;
mod accelstructure;
mod pipeline;

pub fn global_resource_option() -> MTLResourceOptions {
    MTLResourceOptions::StorageModeManaged
}