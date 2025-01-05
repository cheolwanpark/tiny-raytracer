use metal::{ComputePipelineDescriptor, ComputePipelineState, Device, Function, FunctionConstantValues, FunctionRef, Library, LinkedFunctions, LinkedFunctionsRef, MTLDataType};

use crate::hittable::world::World;

const MAIN_KERNEL_NAME: &str = "raytracingKernel";
const INTERSECTION_FUNCTION_NAMES: [&str; 2] = [
    "sphereIntersectionFunction",
    "quadIntersectionFunction"
];

pub struct MetalRaytracingPipeline {
    pipeline: ComputePipelineState,
}

impl MetalRaytracingPipeline {
    pub fn setup(
        device: &Device,
        library: &Library,
    ) -> Self {
        let raytracing_function = library.get_function(MAIN_KERNEL_NAME, None).expect("failed to get raytracing kernel");
        let intersection_functions: Vec<Function> = INTERSECTION_FUNCTION_NAMES
            .iter()
            .map(|function_name| library.get_function(function_name, None).expect("failed to get an intersection function"))
            .collect();
        let intersection_functions: Vec<&FunctionRef> = intersection_functions
            .iter()
            .map(|f| -> &FunctionRef { f })
            .collect();
        let linked_functions = LinkedFunctions::new();
        linked_functions.set_functions(&intersection_functions);
        let pipeline_descriptor = ComputePipelineDescriptor::new();
        pipeline_descriptor.set_compute_function(Some(&raytracing_function));
        pipeline_descriptor.set_linked_functions(linked_functions.as_ref());
        pipeline_descriptor.set_thread_group_size_is_multiple_of_thread_execution_width(true);
        let pipeline = device.new_compute_pipeline_state(&pipeline_descriptor).expect("failed to set pipeline state");
        Self {
            pipeline
        }
    }
}

impl From<MetalRaytracingPipeline> for ComputePipelineState {
    fn from(value: MetalRaytracingPipeline) -> Self {
        value.pipeline
    }
}