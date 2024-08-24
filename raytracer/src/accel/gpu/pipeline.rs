use wgpu;

use super::{buffer::Buffer, instance::Instance, module::Module};

pub struct Pipeline<'a> {
    pub(super) pipeline: wgpu::ComputePipeline,
    instance: &'a Instance,
    pub(super) bind_groups: Vec<(u32, wgpu::BindGroup)>
}

pub struct PipelineOption<'a, 'b> {
    label: Option<&'a str>,
    module: &'b Module,
}

pub struct BindGroup<'b> {
    idx: u32,
    entries: Vec<wgpu::BindGroupEntry<'b>>,
}

pub struct BindGroupEntry<'a>(wgpu::BindGroupEntry<'a>);

impl<'a> Pipeline<'a> {
    fn new<'b, 'c>(instance: &'a Instance, option: PipelineOption<'b, 'c>) -> Self {
        let desc = wgpu::ComputePipelineDescriptor {
            label: option.label,
            module: &option.module,
            layout: None,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None
        };
        let pipeline = instance.device.create_compute_pipeline(&desc);
        Self { pipeline, instance, bind_groups: vec![] }
    }

    pub fn add_bind_group<'b>(&mut self, bindgroup: BindGroup<'b>) {
        let idx = bindgroup.idx;
        let layout = self.pipeline.get_bind_group_layout(idx);
        let desc = wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: bindgroup.entries.as_slice(),
        };
        let bindgroup = self.instance.device.create_bind_group(&desc);
        self.bind_groups.push((idx, bindgroup));
    }
}

impl Instance {
    pub fn create_pipeline<'a, 'b, 'c>(&'a self, option: PipelineOption<'b, 'c>) -> Pipeline<'a> {
        Pipeline::new(&self, option)
    }
}

impl<'a : 'b, 'b : 'a> PipelineOption<'a, 'b> {
    pub fn new(module: &'a Module) -> Self {
        Self {
            module,
            label: None
        }
    }

    pub fn label(self, label: &'b str) -> Self {
        Self {
            module: self.module,
            label: Some(label)
        }
    }
}

impl<'a> BindGroup<'a> {
    pub fn new(idx: u32) -> Self {
        Self { 
            idx,
            entries: vec![],
         }
    }

    pub fn add(mut self, entry: BindGroupEntry<'a>) -> Self {
        self.entries.push(entry.0);
        self
    }
}

impl<'a> BindGroupEntry<'a> {
    pub(super) fn new(idx: u32, resource: wgpu::BindingResource<'a>) -> Self {
        Self(wgpu::BindGroupEntry {
            binding: idx,
            resource: resource
        })
    }
}