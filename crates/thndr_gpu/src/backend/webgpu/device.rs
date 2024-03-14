use crate::interface::{buffer, device};

use super::surface;

/// The WebGPU device.
#[derive(Debug)]
pub struct Device {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Device {
    /// Creates a new [Device] with the given [DeviceConfig].
    pub async fn new<'a>(
        config: device::DeviceConfig,
        instance: &wgpu::Instance,
        compatible_surface: Option<&surface::Surface<'a>>,
    ) -> device::Result<Self> {
        let adapter = Self::create_adapter(instance, &config, compatible_surface).await?;

        let (device, queue) = Self::create_device(&adapter, &config).await?;

        Ok(Self {
            adapter,
            device,
            queue,
        })
    }

    /// Creates the adapter to use for the device.
    async fn create_adapter<'a>(
        instance: &wgpu::Instance,
        config: &device::DeviceConfig,
        compatible_surface: Option<&surface::Surface<'a>>,
    ) -> device::Result<wgpu::Adapter> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: match config.power_preference {
                    device::PowerPreference::Low => wgpu::PowerPreference::LowPower,
                    device::PowerPreference::High => wgpu::PowerPreference::HighPerformance,
                },
                force_fallback_adapter: false,
                compatible_surface: compatible_surface.map(|s| &s.wgpu_surface),
            })
            .await
            .ok_or(device::DeviceError::NoSuitableDevice)?;

        Ok(adapter)
    }

    /// Creates the device and queue to use for the device.
    async fn create_device(
        adapter: &wgpu::Adapter,
        _config: &device::DeviceConfig,
    ) -> device::Result<(wgpu::Device, wgpu::Queue)> {
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    #[cfg(not(target_arch = "wasm32"))]
                    required_limits: wgpu::Limits::default(),
                    #[cfg(target_arch = "wasm32")]
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await
            .map_err(|_| device::DeviceError::InitializationFailed)?;

        Ok((device, queue))
    }
}

impl device::DeviceFunctions for Device {
    fn create_buffer(
        &self,
        descriptor: buffer::BufferDescriptor,
    ) -> buffer::Result<buffer::Buffer> {
        Ok(buffer::Buffer::WebGPU(super::buffer::Buffer::new(
            descriptor,
            &self.device,
        )?))
    }

    fn create_buffer_with_data(
        &self,
        descriptor: buffer::BufferDataDescriptor<'_>,
    ) -> buffer::Result<buffer::Buffer> {
        Ok(buffer::Buffer::WebGPU(
            super::buffer::Buffer::new_with_data(descriptor, &self.device)?,
        ))
    }

    fn info(&self) -> String {
        let info = self.adapter.get_info();

        format!(
            "Device Name: {}\nDevice Type: {:?}",
            info.name, info.device_type
        )
    }
}
