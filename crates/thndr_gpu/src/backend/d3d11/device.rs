use windows::Win32::Foundation as foundation;
use windows::Win32::Graphics::Direct3D as d3d;
use windows::Win32::Graphics::Direct3D11 as d3d11;

use crate::interface::device;

/// The Direct3D 11 device.
#[derive(Debug)]
pub struct Device {
    /// The device configuration.
    pub config: device::DeviceConfig,
    /// The Direct3D 11 device.
    pub device: d3d11::ID3D11Device,
}

impl Device {
    /// Creates a new [Device] with the given [device::DeviceConfig].
    ///
    /// # Safety
    /// This should be safe as long as the given [device::DeviceConfig] is valid.
    pub unsafe fn new(config: device::DeviceConfig) -> device::Result<Self> {
        let mut device = None;

        unsafe {
            d3d11::D3D11CreateDevice(
                None,
                d3d::D3D_DRIVER_TYPE_HARDWARE,
                None,
                d3d11::D3D11_CREATE_DEVICE_DEBUG,
                None,
                d3d11::D3D11_SDK_VERSION,
                Some(&mut device as *mut _),
                None,
                None,
            )
            .map_err(|err| match err.code() {
                foundation::D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS => {
                    device::DeviceError::InitializationFailed
                }
                foundation::E_OUTOFMEMORY => device::DeviceError::OutOfMemory,
                foundation::E_INVALIDARG => device::DeviceError::InitializationFailed,
                foundation::E_NOINTERFACE => device::DeviceError::InitializationFailed,
                foundation::E_NOTIMPL => device::DeviceError::InitializationFailed,
                foundation::E_FAIL => device::DeviceError::InitializationFailed,
                _ => device::DeviceError::InitializationFailed,
            })?;
        }

        if let Some(device) = device {
            Ok(Self { config, device })
        } else {
            Err(device::DeviceError::InitializationFailed)
        }
    }
}
