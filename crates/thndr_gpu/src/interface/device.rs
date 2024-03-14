use thiserror::Error;

use crate::backend::webgpu;

use super::buffer;

/// Represents the power preference of a [Device].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerPreference {
    /// Low power preference.
    Low,
    /// High performance preference.
    High,
}

/// The configuration for creating a new [Device].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceConfig {
    /// The power preference of the device.
    pub power_preference: PowerPreference,
}

/// All possible errors that can occur when creating a [Device].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DeviceError {
    /// Not enough memory.
    #[error("Not enough memory.")]
    OutOfMemory,
    /// The device is lost.
    #[error("The device is lost.")]
    DeviceLost,
    /// The device does not support the required features.
    #[error("The device does not support the required features.")]
    UnsupportedFeatures,
    /// The device does not support the required extensions.
    #[error("The device does not support the required extensions.")]
    UnsupportedExtensions,
    /// There is no suitable device.
    #[error("There is no suitable device.")]
    NoSuitableDevice,
    /// The initialisation of the device failed.
    #[error("The initialisation of the device failed.")]
    InitializationFailed,
}

/// The result of creating a [Device].
pub type Result<T, E = DeviceError> = std::result::Result<T, E>;

/// The required functionality for a [Device].
pub trait DeviceFunctions {
    /// Creates a new [buffer::Buffer] with the given [buffer::BufferDescriptor].
    fn create_buffer(&self, descriptor: buffer::BufferDescriptor)
        -> buffer::Result<buffer::Buffer>;

    /// Creates a new [buffer::Buffer] with the given [buffer::BufferDataDescriptor] and initial data.
    fn create_buffer_with_data(
        &self,
        descriptor: buffer::BufferDataDescriptor<'_>,
    ) -> buffer::Result<buffer::Buffer>;

    /// Get a [String] with some information about the [Device].
    fn info(&self) -> String;
}

/// A [Device] allows for direct interaction with the GPU.
#[derive(Debug)]
pub enum Device {
    #[cfg(feature = "webgpu")]
    /// The WebGPU device.
    WebGPU(webgpu::device::Device),
}

impl Device {
    /// Create a new [buffer::Buffer] with the given [buffer::BufferDescriptor].
    pub fn create_buffer(
        &self,
        descriptor: buffer::BufferDescriptor,
    ) -> buffer::Result<buffer::Buffer> {
        match self {
            #[cfg(feature = "webgpu")]
            Device::WebGPU(device) => device.create_buffer(descriptor),
        }
    }

    /// Create a new [buffer::Buffer] with the given [buffer::BufferDataDescriptor] and initial data.
    pub fn create_buffer_with_data(
        &self,
        descriptor: buffer::BufferDataDescriptor<'_>,
    ) -> buffer::Result<buffer::Buffer> {
        match self {
            #[cfg(feature = "webgpu")]
            Device::WebGPU(device) => device.create_buffer_with_data(descriptor),
        }
    }

    /// Get a [String] with some information about the [Device].
    pub fn info(&self) -> String {
        match self {
            #[cfg(feature = "webgpu")]
            Device::WebGPU(device) => device.info(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{buffer, device, instance};

    #[cfg(feature = "webgpu")]
    #[test]
    fn test_create_buffer() {
        let instance = instance::Instance::new(instance::InstanceConfig {
            backend: crate::backend::Backend::WebGPU,
            app_name: "Test".to_string(),
            app_version: instance::Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        })
        .unwrap();

        let device = instance
            .create_device(
                device::DeviceConfig {
                    power_preference: device::PowerPreference::Low,
                },
                None,
            )
            .unwrap();

        let buffer = device
            .create_buffer(buffer::BufferDescriptor {
                size: 1024,
                usage: buffer::BufferUsage::Vertex,
            })
            .unwrap();

        assert!(matches!(buffer, buffer::Buffer::WebGPU(_)));
    }

    #[cfg(feature = "webgpu")]
    #[test]
    fn test_create_buffer_with_data() {
        let instance = instance::Instance::new(instance::InstanceConfig {
            backend: crate::backend::Backend::WebGPU,
            app_name: "Test".to_string(),
            app_version: instance::Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        })
        .unwrap();

        let device = instance
            .create_device(
                device::DeviceConfig {
                    power_preference: device::PowerPreference::Low,
                },
                None,
            )
            .unwrap();

        let buffer = device
            .create_buffer_with_data(buffer::BufferDataDescriptor {
                usage: buffer::BufferUsage::Vertex,
                data: &[0, 1, 2, 3],
            })
            .unwrap();

        assert!(matches!(buffer, buffer::Buffer::WebGPU(_)));
    }
}
