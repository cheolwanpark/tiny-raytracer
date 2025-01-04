use metal::{AccelerationStructureGeometryDescriptor, MTLResourceOptions};

pub trait MetalGeometry {
    fn upload_to_buffers(&mut self);
    fn get_geometry_descriptor(&self) -> AccelerationStructureGeometryDescriptor;
    fn get_intersection_function_name(&self) -> Option<&str>;
    fn get_mask(&self) -> u32;
}

pub mod sphere;
pub mod quad;
pub mod masks;