use metal::{AccelerationStructureGeometryDescriptor, MTLResourceOptions};

pub trait MetalGeometry {
    fn upload_to_buffers(&mut self);
    fn get_geometry_descriptor(&self) -> AccelerationStructureGeometryDescriptor;
    fn get_intersection_function_name(&self) -> Option<&str>;
    fn get_resource_option(&self) -> MTLResourceOptions {
        MTLResourceOptions::StorageModeManaged
    }
}

pub mod sphere;
pub mod quad;