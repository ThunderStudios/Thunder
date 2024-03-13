use thiserror::Error;

use crate::backend::{d3d11, vulkan};

/// The configuration for creating a new [Device].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceConfig {} // empty for now, will hold things like what kind of queue families to create

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
pub trait DeviceFunctions {}

/// A [Device] allows for direct interaction with the GPU.
#[derive(Debug)]
pub enum Device {
    #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
    /// The Vulkan device.
    Vulkan(vulkan::device::Device),
    #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
    /// The Direct3D 11 device.
    D3D11(d3d11::device::Device),
}
