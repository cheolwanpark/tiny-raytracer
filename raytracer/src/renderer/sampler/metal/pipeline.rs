use std::collections::BTreeMap;

use metal::{ComputePipelineDescriptor, ComputePipelineState, Device, Function, FunctionConstantValues, FunctionRef, IntersectionFunctionTable, IntersectionFunctionTableDescriptor, Library, LinkedFunctions, LinkedFunctionsRef, MTLDataType};

use crate::hittable::world::World;

const MAIN_KERNEL_NAME: &str = "raytracingKernel";
const INTERSECTION_FUNCTION_NAMES: [&str; 2] = [
    "quadIntersectionFunction",
    "sphereIntersectionFunction",
];

pub struct MetalRaytracingPipeline {
    pipeline: ComputePipelineState,
    intersection_functions: BTreeMap<String, Function>,
}

impl MetalRaytracingPipeline {
    pub fn new(
        device: &Device,
        library: &Library,
    ) -> Self {
        let raytracing_function = library.get_function(MAIN_KERNEL_NAME, None).expect("failed to get raytracing kernel");
        let mut intersection_functions = BTreeMap::new();
        INTERSECTION_FUNCTION_NAMES.iter().for_each(|function_name| {
            let func = library.get_function(function_name, None).expect("failed to get an intersection function");
            intersection_functions.insert(function_name.to_string(), func);
        });
        let intersection_functions_vec: Vec<&FunctionRef> = intersection_functions
            .iter()
            .map(|(_, func)| func.as_ref())
            .collect();
        let linked_functions = LinkedFunctions::new();
        linked_functions.set_functions(&intersection_functions_vec);
        let pipeline_descriptor = ComputePipelineDescriptor::new();
        pipeline_descriptor.set_compute_function(Some(&raytracing_function));
        pipeline_descriptor.set_linked_functions(linked_functions.as_ref());
        pipeline_descriptor.set_thread_group_size_is_multiple_of_thread_execution_width(true);
        let pipeline = device.new_compute_pipeline_state(&pipeline_descriptor).expect("failed to set pipeline state");
        
        Self {
            pipeline,
            intersection_functions,
        }
    }

    pub fn setup_intersection_function_table(&self, device: &Device, world: &World) -> IntersectionFunctionTable {
        let geometries = world.get_metal_geometries(device.clone());

        let function_table_descriptor = IntersectionFunctionTableDescriptor::new();
        function_table_descriptor.set_function_count(INTERSECTION_FUNCTION_NAMES.len() as u64);
        let intersection_function_table = self.pipeline.new_intersection_function_table_with_descriptor(&function_table_descriptor);

        geometries.iter().enumerate().for_each(|(i, geometry)| {
            if let Some(func_name) = geometry.get_intersection_function_name() {
                let func = self.intersection_functions.get(func_name).unwrap();
                let handle = self.pipeline
                    .function_handle_with_function(func.as_ref())
                    .unwrap();
                intersection_function_table.set_function(handle, i as u64);
            }
        });

        intersection_function_table
    }

    pub fn state(&self) -> &ComputePipelineState {
        &self.pipeline
    }
}

impl From<MetalRaytracingPipeline> for ComputePipelineState {
    fn from(value: MetalRaytracingPipeline) -> Self {
        value.pipeline
    }
}