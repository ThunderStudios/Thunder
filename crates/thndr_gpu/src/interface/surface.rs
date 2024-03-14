use std::sync::Arc;

use thiserror::Error;

/// A marker trait for any window handle.
pub trait WindowHandle:
    raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle + Send + Sync
{
}

use crate::backend::webgpu;

/// The error type used by the [Surface] type.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SurfaceError {
    /// The backend failed to create the surface.
    #[error("The backend failed to create the surface.")]
    BackendError,
    /// Failed to configure the surface.
    #[error("Failed to configure the surface.")]
    ConfigureError,
}

/// The result type used by the [Surface] type.
pub type Result<T, E = SurfaceError> = std::result::Result<T, E>;

/// The configuration for creating a new [Surface].
#[derive(Clone, PartialEq, Eq)]
pub struct SurfaceConfig<W: WindowHandle> {
    /// The raw window handle.
    pub raw_window_handle: Arc<W>,
}

/// A [Surface] allows for drawing to a window or other surface.
#[derive(Debug)]
pub enum Surface<'surface> {
    #[cfg(feature = "webgpu")]
    /// The WebGPU surface.
    WebGPU(webgpu::surface::Surface<'surface>),
}
