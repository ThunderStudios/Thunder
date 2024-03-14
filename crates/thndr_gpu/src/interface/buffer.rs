use thiserror::Error;

use crate::backend::webgpu;

/// All buffer-related errors.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BufferError {
    /// Not enough memory.
    #[error("Not enough memory.")]
    OutOfMemory,
}

/// The result of a buffer operation.
pub type Result<T, E = BufferError> = std::result::Result<T, E>;

/// The usage of the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    /// The buffer is used as a vertex buffer.
    Vertex,
    /// The buffer is used as an index buffer.
    Index,
    /// The buffer is used as a uniform buffer.
    Uniform,
    /// The buffer is used as a storage buffer.
    Storage,
}

/// The descriptor for creating a new [Buffer].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferDescriptor {
    /// The size of the buffer in bytes.
    pub size: u64,
    /// The usage of the buffer.
    pub usage: BufferUsage,
}

/// The descriptor for creating a new [Buffer] with data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferDataDescriptor<'data> {
    /// The usage of the buffer.
    pub usage: BufferUsage,
    /// The data to initialise the buffer with.
    pub data: &'data [u8],
}

/// A [Buffer] is used to store data on the GPU.
#[derive(Debug)]
pub enum Buffer {
    #[cfg(feature = "webgpu")]
    /// The WebGPU buffer.
    WebGPU(webgpu::buffer::Buffer),
}
