use super::{
    buffer::{Buffer, BufferOption, BufferUsages}, 
    instance::{Instance, InstanceOption}, 
    module::Module, 
    pipeline::{Pipeline, PipelineOption, BindGroup},
    encoder::CommandEncoder,
};

#[tokio::test]
async fn create_instance() {
    let instance = Instance::new(InstanceOption::new()).await;
    assert!(instance.is_some(), "Instance creation failed");
}

#[tokio::test]
async fn create_buffer() {
    let instance = Instance::new(InstanceOption::new()).await.unwrap();
    let data = vec![0u8; 128];
    let _buffer = instance.create_buffer(BufferOption::new(128).label("Test Sized Buffer"));
    let _buffer = instance.create_buffer(BufferOption::new_with_data(&data).label("Test Buffer with Data"));
}

fn create_dummy_module(instance: &Instance) -> Module {
    let dummy_source = "
    @group(0) @binding(0) var<storage, read_write> data: array<f32>;

    @compute @workgroup_size(1) fn main(@builtin(global_invocation_id) id: vec3u) {
        let i = id.x;
        data[i] = data[i] * 2.0;
    }
    ";
    instance.create_module_with_label("Test Shader", &dummy_source)
}

#[tokio::test]
async fn create_module() {
    let instance = Instance::new(InstanceOption::new()).await.unwrap();
    let _module = create_dummy_module(&instance);
}

#[tokio::test]
async fn create_encoder() {
    let instance = Instance::new(InstanceOption::new()).await.unwrap();
    let _encoder = instance.create_command_encoder();
}

#[tokio::test]
async fn create_pipeline() {
    let instance = Instance::new(InstanceOption::new()).await.unwrap();

    let module = create_dummy_module(&instance);
    let mut pipeline = instance.create_pipeline(PipelineOption::new(&module).label("Test Pipeline"));

    let data = vec![1.0f32; 64];
    let data_buffer = instance.create_buffer(BufferOption::new_with_data(data.as_slice()));

    pipeline.add_bind_group(BindGroup::new(0)
                            .add(data_buffer.as_binding(0)));
}

#[tokio::test]
async fn simple_calculation_test() {
    let instance = Instance::new(InstanceOption::new()).await.unwrap();

    let mut data = vec![0.0f32; 128];
    for i in 1..=128 {
        data[i-1] = i as f32;
    }
    let data_buffer = instance.create_buffer(
        BufferOption::new_with_data(&data).label("Simple Calculation Test Data")
    );
    let read_buffer = instance.create_buffer(
        BufferOption::new(data_buffer.size()).usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
    );
    
    let module = create_dummy_module(&instance);
    let mut pipeline = instance.create_pipeline(PipelineOption::new(&module).label("Test Pipeline"));
    pipeline.add_bind_group(BindGroup::new(0)
                            .add(data_buffer.as_binding(0)));

    let mut encoder = instance.create_command_encoder();
    {
        let mut cpass = encoder.create_compute_pass();
        cpass.set_pipeline(pipeline);
        cpass.dispatch_workgroups(data.len() as u32, 1, 1);
    }
    encoder.copy_buffer(&data_buffer, &read_buffer);

    instance.execute(encoder);

    let read_buffer_slice = read_buffer.slice(..);
    read_buffer_slice.map_read();

    instance.wait();

    let result = read_buffer_slice.get_mapped_range();
    let result: Vec<f32> = result.as_slice().to_vec();
    for i in 1..=128 {
        assert_eq!(result[i-1], 2.0*i as f32);
    }
}