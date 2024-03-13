use crate::interface::{
    self,
    instance::{self, InstanceConfig},
};

use super::device;

/// The Direct3D 11 instance.
#[derive(Debug)]
pub struct Instance {
    /// The instance configuration.
    pub config: InstanceConfig,
}

impl instance::InstanceFunctions for Instance {
    fn new(config: InstanceConfig) -> instance::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { config })
    }

    fn create_device(
        _this: std::sync::Arc<Self>,
        config: interface::device::DeviceConfig,
    ) -> interface::device::Result<interface::device::Device> {
        Ok(interface::device::Device::D3D11(unsafe {
            device::Device::new(config)
        }?))
    }
}
