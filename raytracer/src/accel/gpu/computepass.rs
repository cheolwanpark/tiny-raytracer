use wgpu;

use super::{encoder::CommandEncoder, pipeline::Pipeline};

pub struct ComputePass<'a>(wgpu::ComputePass<'a>);

impl<'a> ComputePass<'a> {
    fn new(encoder: &'a mut CommandEncoder) -> Self {
        let desc = wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        };
        let cpass = encoder.0.begin_compute_pass(&desc);
        Self(cpass)
    }

    fn new_with_label<'b>(encoder: &'a mut CommandEncoder, label: &'b str) -> Self {
        let desc = wgpu::ComputePassDescriptor {
            label: Some(label),
            timestamp_writes: None,
        };
        let cpass = encoder.0.begin_compute_pass(&desc);
        Self(cpass)
    }

    pub fn set_pipeline(&mut self, pipeline: Pipeline) {
        self.0.set_pipeline(&pipeline.pipeline);
        for (idx, bind_group) in pipeline.bind_groups {
            self.0.set_bind_group(idx, &bind_group, &[]);
        }
    }

    pub fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
        self.0.dispatch_workgroups(x, y, z);
    }
}

impl CommandEncoder {
    pub fn create_compute_pass<'a>(&'a mut self) -> ComputePass<'a> {
        ComputePass::new(self)
    }
}