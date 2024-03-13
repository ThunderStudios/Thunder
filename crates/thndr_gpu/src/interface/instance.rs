use thiserror::Error;

use crate::backend::{d3d11, vulkan, Backend};
use std::{fmt::Debug, sync::Arc};

use super::device;

/// The version of an application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    /// The major version.
    pub major: u32,
    /// The minor version.
    pub minor: u32,
    /// The patch version.
    pub patch: u32,
}

/// The configuration for creating a new [Instance].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceConfig {
    /// The backend to use for the [Instance].
    pub backend: Backend,
    /// The name of the application.
    pub app_name: String,
    /// The version of the application.
    pub app_version: Version,
}

/// All possible errors that can occur when creating an [Instance].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InstanceError {
    /// The backend is not supported.
    #[error("The backend is not supported.")]
    UnsupportedBackend,
    /// The backend failed to create the instance.
    #[error("The backend failed to create the instance.")]
    BackendError,
}

/// The result of creating an [Instance].
pub type Result<T, E = InstanceError> = std::result::Result<T, E>;

/// An [Instance] allows for the creation of [device::Device]s and [super::surface::Surface]s.
#[derive(Debug)]
pub enum Instance {
    #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
    /// The Vulkan instance.
    Vulkan(Arc<vulkan::instance::Instance>),
    #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
    /// The Direct3D 11 instance.
    D3D11(Arc<d3d11::instance::Instance>),
}

/// The required functionality for an [Instance].
pub trait InstanceFunctions {
    /// Creates a new [Instance] with the given [InstanceConfig].
    fn new(config: InstanceConfig) -> Result<Self>
    where
        Self: Sized;

    /// Creates a new [device::Device] using the given [device::DeviceConfig].
    #[cfg_attr(
        feature = "vulkan",
        doc = "In the case of the Vulkan backend, this will also pick the ideal physical device."
    )]
    fn create_device(
        this: Arc<Self>,
        config: device::DeviceConfig,
    ) -> device::Result<device::Device>;
}

impl Instance {
    /// Creates a new [Instance] with the given [InstanceConfig].
    pub fn new(config: InstanceConfig) -> Result<Self> {
        match config.backend {
            #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
            Backend::Vulkan => {
                let instance = vulkan::instance::Instance::new(config)?;
                Ok(Instance::Vulkan(Arc::new(instance)))
            }
            #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
            Backend::D3D11 => {
                let instance = d3d11::instance::Instance::new(config)?;
                Ok(Instance::D3D11(Arc::new(instance)))
            }
            #[cfg(all(feature = "d3d12", target_os = "windows", not(target_arch = "wasm32")))]
            Backend::D3D12 => todo!(),
        }
    }

    /// Creates a new [device::Device] using the given [device::DeviceConfig].
    pub fn create_device(&self, config: device::DeviceConfig) -> device::Result<device::Device> {
        match self {
            #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
            Instance::Vulkan(instance) => {
                vulkan::instance::Instance::create_device(instance.clone(), config)
            }
            #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
            Instance::D3D11(instance) => {
                d3d11::instance::Instance::create_device(instance.clone(), config)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
    #[test]
    fn create_instance_vulkan() {
        let config = InstanceConfig {
            backend: Backend::Vulkan,
            app_name: "Test".to_string(),
            app_version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        };

        let instance = Instance::new(config).unwrap();
        println!("{:?}", instance);
    }

    #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
    #[test]
    fn create_instance_d3d11() {
        let config = InstanceConfig {
            backend: Backend::D3D11,
            app_name: "Test".to_string(),
            app_version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        };

        let instance = Instance::new(config).unwrap();
        println!("{:?}", instance);
    }

    #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
    #[test]
    fn create_device_vulkan() {
        let config = InstanceConfig {
            backend: Backend::Vulkan,
            app_name: "Test".to_string(),
            app_version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        };

        let instance = Instance::new(config).unwrap();
        let device = instance.create_device(device::DeviceConfig {}).unwrap();
        println!("{:?}", device);
    }

    #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
    #[test]
    fn create_device_d3d11() {
        let config = InstanceConfig {
            backend: Backend::D3D11,
            app_name: "Test".to_string(),
            app_version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        };

        let instance = Instance::new(config).unwrap();
        let device = instance.create_device(device::DeviceConfig {}).unwrap();
        println!("{:?}", device);
    }
}
