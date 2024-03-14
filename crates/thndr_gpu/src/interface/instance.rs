use thiserror::Error;

use crate::backend::{webgpu, Backend};
use std::{fmt::Debug, sync::Arc};

use super::{device, surface};

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
    #[cfg(feature = "webgpu")]
    /// The WebGPU instance.
    WebGPU(Arc<webgpu::instance::Instance>),
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
        compatible_surface: Option<&surface::Surface>,
    ) -> device::Result<device::Device>;

    /// Creates a new [surface::Surface] using the given [surface::SurfaceConfig].
    fn create_surface<'a, W: surface::WindowHandle + 'a>(
        this: Arc<Self>,
        config: surface::SurfaceConfig<W>,
    ) -> surface::Result<surface::Surface<'a>>;
}

impl Instance {
    /// Creates a new [Instance] with the given [InstanceConfig].
    pub fn new(config: InstanceConfig) -> Result<Self> {
        match config.backend {
            #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
            Backend::Vulkan => todo!(),
            #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
            Backend::D3D11 => todo!(),
            #[cfg(feature = "webgpu")]
            Backend::WebGPU => {
                let instance = webgpu::instance::Instance::new(config)?;
                Ok(Instance::WebGPU(Arc::new(instance)))
            }
        }
    }

    /// Creates a new [device::Device] using the given [device::DeviceConfig].
    pub fn create_device(
        &self,
        config: device::DeviceConfig,
        compatible_surface: Option<&surface::Surface>,
    ) -> device::Result<device::Device> {
        match self {
            #[cfg(feature = "webgpu")]
            Instance::WebGPU(instance) => webgpu::instance::Instance::create_device(
                instance.clone(),
                config,
                compatible_surface,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "webgpu")]
    #[test]
    fn test_webgpu_instance() {
        let instance = Instance::new(InstanceConfig {
            backend: Backend::WebGPU,
            app_name: "Test".to_string(),
            app_version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        })
        .unwrap();

        let device = instance
            .create_device(
                device::DeviceConfig {
                    power_preference: device::PowerPreference::High,
                },
                None,
            )
            .unwrap();

        assert!(matches!(device, device::Device::WebGPU(_)));
    }
}
