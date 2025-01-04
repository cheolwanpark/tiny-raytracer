use metal::MTLResourceOptions;

pub mod geometry;
mod sampler;
mod accelstructure;

pub fn global_resource_option() -> MTLResourceOptions {
    MTLResourceOptions::StorageModeManaged
}