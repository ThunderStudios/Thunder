use std::{ffi::CString, fmt::Debug, sync::Arc};

use ash::{vk, Entry};

use crate::interface::{
    device,
    instance::{self, InstanceConfig},
};

/// Attempt to load the local Vulkan library.
///
/// # Safety
///
/// This function is unsafe because it loads the Vulkan library,
/// and dynamically loading libraries is inherently unsafe.
/// Especially in multi-threaded environments, but this setup is single threaded, so we should be fine.
unsafe fn load_vulkan() -> Result<ash::Entry, instance::InstanceError> {
    unsafe { Entry::load().map_err(|_| instance::InstanceError::BackendError) }
}

/// The vulkan Instance.
pub struct Instance {
    /// The Vulkan instance.
    pub vk_instance: ash::Instance,
    /// The Vulkan entry point.
    pub entry: Entry,
    /// The instance config.
    pub config: InstanceConfig,
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.vk_instance.destroy_instance(None); // TODO: Allocation callbacks.
        }
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("config", &self.config)
            .finish()
    }
}

impl instance::InstanceFunctions for Instance {
    fn new(config: InstanceConfig) -> instance::Result<Self> {
        let app_name = CString::new(config.app_name.clone()).unwrap();
        let engine_name = CString::new("Thunder").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(vk::make_api_version(
                0,
                config.app_version.major,
                config.app_version.minor,
                config.app_version.patch,
            ))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 0, 1, 0)); // TODO: Make this automatic.

        let create_info = vk::InstanceCreateInfo::builder().application_info(&app_info);

        let entry = unsafe { load_vulkan()? };

        let vk_instance = unsafe { Self::create_instance(&entry, &create_info)? };

        Ok(Self {
            vk_instance,
            entry,
            config,
        })
    }

    fn create_device(
        this: Arc<Self>,
        config: device::DeviceConfig,
    ) -> device::Result<device::Device> {
        let physical_device = this.pick_physical_device()?;

        // SAFETY: This is safe because the physical device is valid.
        Ok(device::Device::Vulkan(unsafe {
            super::device::Device::new(this, physical_device, &config)?
        }))
    }
}

impl Instance {
    /// Creates the internal [ash::Instance].
    ///
    /// # Safety
    /// The returned instance must be destroyed before the [Entry] is dropped.
    /// Which is handled if they are both put into the [Instance] struct.
    pub unsafe fn create_instance(
        entry: &Entry,
        create_info: &vk::InstanceCreateInfo,
    ) -> Result<ash::Instance, instance::InstanceError> {
        unsafe {
            entry
                .create_instance(create_info, None)
                .map_err(|_| instance::InstanceError::BackendError)
        }
    }

    /// Rate the suitability of a physical device.
    ///
    /// # Safety
    /// Caller must ensure the given physical device is valid. (if it was optained from the instance, it should be valid.)
    unsafe fn rate_device_suitability(&self, device: vk::PhysicalDevice) -> i32 {
        let properties = unsafe { self.vk_instance.get_physical_device_properties(device) };
        let _features = unsafe { self.vk_instance.get_physical_device_features(device) };

        let mut score = 0;

        match properties.device_type {
            vk::PhysicalDeviceType::DISCRETE_GPU => score += 1000,
            vk::PhysicalDeviceType::INTEGRATED_GPU => score += 100,
            vk::PhysicalDeviceType::CPU => score += 10,
            vk::PhysicalDeviceType::VIRTUAL_GPU => score += 1,
            vk::PhysicalDeviceType::OTHER => score += 0,
            _ => score += 0,
        }

        score
    }

    /// Pick the best physical device.
    pub fn pick_physical_device(&self) -> device::Result<vk::PhysicalDevice> {
        // SAFETY: This is safe because we are only reading the physical devices.
        let physical_devices = unsafe {
            self.vk_instance
                .enumerate_physical_devices()
                .map_err(|_| device::DeviceError::InitializationFailed)?
        };

        let mut best_score = 0;
        let mut best_device = vk::PhysicalDevice::null();

        for &device in physical_devices.iter() {
            // SAFETY: This is safe since the physical device is valid.
            let score = unsafe { self.rate_device_suitability(device) };
            if score > best_score {
                best_score = score;
                best_device = device;
            }
        }

        if best_device == vk::PhysicalDevice::null() {
            Err(device::DeviceError::NoSuitableDevice)
        } else {
            Ok(best_device)
        }
    }
}
