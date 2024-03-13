use std::{fmt::Debug, sync::Arc};

use ash::vk;

use super::instance;
use crate::interface::device;

/// The vulkan device.
pub struct Device {
    device: ash::Device,

    /// The instance this device was created from. (mainly here to order cleanup properly).
    pub instance: Arc<instance::Instance>,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None); // TODO: Allocation callbacks.
        }
    }
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device").finish()
    }
}

impl Device {
    /// Creates a new device.
    pub unsafe fn new(
        instance: Arc<instance::Instance>,
        physical_device: vk::PhysicalDevice,
        _config: &device::DeviceConfig,
    ) -> device::Result<Self> {
        let create_info = vk::DeviceCreateInfo::builder().build();

        let device = unsafe {
            instance
                .vk_instance
                .create_device(physical_device, &create_info, None)
                .map_err(|res| match res {
                    vk::Result::ERROR_OUT_OF_HOST_MEMORY => device::DeviceError::OutOfMemory,
                    vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => device::DeviceError::OutOfMemory,
                    vk::Result::ERROR_INITIALIZATION_FAILED => {
                        device::DeviceError::InitializationFailed
                    }
                    vk::Result::ERROR_DEVICE_LOST => device::DeviceError::DeviceLost,
                    vk::Result::ERROR_FEATURE_NOT_PRESENT => {
                        device::DeviceError::UnsupportedFeatures
                    }
                    vk::Result::ERROR_EXTENSION_NOT_PRESENT => {
                        device::DeviceError::UnsupportedExtensions
                    }
                    vk::Result::ERROR_TOO_MANY_OBJECTS => device::DeviceError::InitializationFailed,
                    _ => device::DeviceError::InitializationFailed,
                })?
        };

        Ok(Self { device, instance })
    }
}
