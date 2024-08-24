use std::ops::RangeBounds;

use bytemuck::{AnyBitPattern, NoUninit};
use wgpu;
use wgpu::util::DeviceExt;

use super::{encoder::CommandEncoder, instance::Instance, pipeline::BindGroupEntry};

pub struct Buffer(wgpu::Buffer);
pub struct BufferSlice<'a>(wgpu::BufferSlice<'a>);
pub struct BufferView<'a>(wgpu::BufferView<'a>);
pub struct BufferViewMut<'a>(wgpu::BufferViewMut<'a>);

pub struct BufferOption<'a, 'b> {
    label: Option<&'a str>,
    data: Option<&'b [u8]>,
    size: u64,
    usage: wgpu::BufferUsages,
}
pub type BufferUsages = wgpu::BufferUsages;

impl Buffer {
    fn new(instance: &Instance, option: BufferOption) -> Self {
        let buffer = if let Some(data) = option.data {
            let desc = wgpu::util::BufferInitDescriptor {
                label: option.label,
                contents: data,
                usage: option.usage,
            };
            instance.device.create_buffer_init(&desc)
        } else {
            let desc = wgpu::BufferDescriptor {
                label: option.label,
                size: option.size,
                usage: option.usage,
                mapped_at_creation: false,
            };
            instance.device.create_buffer(&desc)
        };
        Self(buffer)
    }

    pub fn size(&self) -> u64 {
        self.0.size()
    }

    pub fn as_binding<'a>(&'a self, idx: u32) -> BindGroupEntry<'a> {
        BindGroupEntry::new(idx, self.0.as_entire_binding())
    }

    pub fn slice<'a, B: RangeBounds<u64>>(&'a self, bounds: B) -> BufferSlice<'a> {
        BufferSlice(self.0.slice(bounds))
    }
}

impl Instance {
    pub fn create_buffer(&self, option: BufferOption) -> Buffer {
        Buffer::new(self, option)
    }
}

impl CommandEncoder {
    pub fn copy_buffer(&mut self, src: &Buffer, dest: &Buffer) {
        self.copy_buffer_with_size(src, dest, src.size());
    }

    pub fn copy_buffer_with_size(&mut self, src: &Buffer, dest: &Buffer, size: u64) {
        self.0.copy_buffer_to_buffer(&src.0, 0, &dest.0, 0, size);
    }
}

impl<'a> BufferSlice<'a> {
    pub fn map_read(&self) {
        self.map_read_with_callback(|_v| {});
    }

    pub fn map_read_with_callback(
        &self,
        callback: impl FnOnce(Result<(), wgpu::BufferAsyncError>) + wgpu::WasmNotSend + 'static
    ) {
        self.0.map_async(wgpu::MapMode::Read, callback);
    }

    pub fn map_write(&self) {
        self.map_write_with_callback(|_v| {});
    }

    pub fn map_write_with_callback(
        &self,
        callback: impl FnOnce(Result<(), wgpu::BufferAsyncError>) + wgpu::WasmNotSend + 'static
    ) {
        self.0.map_async(wgpu::MapMode::Write, callback);
    }

    pub fn get_mapped_range(&self) -> BufferView<'a> {
        BufferView(self.0.get_mapped_range())
    }

    pub fn get_mapped_range_mut(&self) -> BufferViewMut<'a> {
        BufferViewMut(self.0.get_mapped_range_mut())
    }
}

impl<'a> BufferView<'a> {
    pub fn as_slice<T: AnyBitPattern>(&self) -> &[T] {
        bytemuck::cast_slice(&self.0)
    }

    pub fn as_data<T: AnyBitPattern>(&self) -> &T {
        bytemuck::from_bytes(&self.0)
    }
}

impl<'a> BufferViewMut<'a> {
    pub fn clone_from_slice<T: AnyBitPattern + NoUninit>(&mut self, slice: &[T]) {
        self.0.clone_from_slice(bytemuck::cast_slice(slice));
    }

    pub fn clone_from_data<T: AnyBitPattern + NoUninit>(&mut self, data: &T) {
        self.0.clone_from_slice(bytemuck::bytes_of(data));
    }
}

impl<'a, 'b> BufferOption<'a, 'b> {
    pub fn new(size: u64) -> Self {
        Self {
            label: None,
            data: None,
            size,
            usage: Self::default_usage(),
        }
    }

    pub fn new_with_data<T: NoUninit>(data: &'b [T]) -> Self {
        let bytes: &'b [u8] = bytemuck::cast_slice(data);
        Self {
            label: None,
            data: Some(bytes),
            size: 0,
            usage: Self::default_usage(),
        }
    }

    pub fn label(self, label: &'a str) -> Self {
        Self {
            label: Some(label),
            data: self.data,
            size: self.size,
            usage: self.usage
        }
    }

    pub fn usage(self, usage: BufferUsages) -> Self {
        Self {
            label: self.label,
            data: self.data,
            size: self.size,
            usage,
        }
    }

    fn default_usage() -> BufferUsages {
        BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC
    }
}