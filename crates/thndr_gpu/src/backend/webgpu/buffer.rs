use wgpu::util::DeviceExt as _;

use crate::interface::buffer::{self, BufferUsage};

/// A WebGPU buffer.
#[derive(Debug)]
pub struct Buffer {
    /// The inner wgpu buffer.
    pub wgpu_buffer: wgpu::Buffer,
}

impl Buffer {
    /// Creates a new [Buffer] with the given [BufferDescriptor].
    pub fn new(
        descriptor: buffer::BufferDescriptor,
        device: &wgpu::Device,
    ) -> buffer::Result<Self> {
        let usage = match descriptor.usage {
            BufferUsage::Vertex => wgpu::BufferUsages::VERTEX,
            BufferUsage::Index => wgpu::BufferUsages::INDEX,
            BufferUsage::Uniform => wgpu::BufferUsages::UNIFORM,
            BufferUsage::Storage => wgpu::BufferUsages::STORAGE,
        };
        let wgpu_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: descriptor.size,
            usage,
            mapped_at_creation: false,
        });
        Ok(Self { wgpu_buffer })
    }

    /// Creates a new [Buffer] with the given [BufferDataDescriptor].
    pub fn new_with_data(
        descriptor: buffer::BufferDataDescriptor<'_>,
        device: &wgpu::Device,
    ) -> buffer::Result<Self> {
        let usage = match descriptor.usage {
            BufferUsage::Vertex => wgpu::BufferUsages::VERTEX,
            BufferUsage::Index => wgpu::BufferUsages::INDEX,
            BufferUsage::Uniform => wgpu::BufferUsages::UNIFORM,
            BufferUsage::Storage => wgpu::BufferUsages::STORAGE,
        };
        let wgpu_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: descriptor.data,
            usage,
        });
        Ok(Self { wgpu_buffer })
    }
}
