use crate::interface::{
    device,
    instance::{self, InstanceConfig},
    surface,
};

/// The WebGPU Instance.
#[derive(Debug)]
pub struct Instance {
    /// The internal wgpu instance.
    pub wgpu_instance: wgpu::Instance,
    /// The instance config.
    pub config: InstanceConfig,
}

impl Instance {
    /// Creates a new [Instance] with the given [InstanceConfig].
    pub fn new(config: InstanceConfig) -> instance::Result<Self> {
        let wgpu_instance = wgpu::Instance::default();
        Ok(Instance {
            wgpu_instance,
            config,
        })
    }
}

impl instance::InstanceFunctions for Instance {
    fn new(config: InstanceConfig) -> instance::Result<Self>
    where
        Self: Sized,
    {
        Self::new(config)
    }

    fn create_device(
        this: std::sync::Arc<Self>,
        config: device::DeviceConfig,
        compatible_surface: Option<&surface::Surface>,
    ) -> device::Result<device::Device> {
        match compatible_surface {
            Some(surface::Surface::WebGPU(surface)) => {
                Ok(device::Device::WebGPU(pollster::block_on(
                    super::device::Device::new(config, &this.wgpu_instance, Some(surface)),
                )?))
            }
            None => Ok(device::Device::WebGPU(pollster::block_on(
                super::device::Device::new(config, &this.wgpu_instance, None),
            )?)),
            _ => Err(device::DeviceError::InitializationFailed),
        }
    }

    fn create_surface<'a, W: surface::WindowHandle + 'a>(
        this: std::sync::Arc<Self>,
        config: surface::SurfaceConfig<W>,
    ) -> surface::Result<surface::Surface<'a>> {
        Ok(surface::Surface::WebGPU(super::surface::Surface::new(
            config,
            &this.wgpu_instance,
        )?))
    }
}
