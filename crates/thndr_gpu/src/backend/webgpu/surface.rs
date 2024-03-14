use crate::interface::surface;

/// The WebGPU surface.
#[derive(Debug)]
pub struct Surface<'surface> {
    /// The inner wgpu surface.
    pub wgpu_surface: wgpu::Surface<'surface>,
}

impl<'surface> Surface<'surface> {
    /// Creates a new [Surface] with the given [surface::SurfaceConfig].
    pub fn new<W: surface::WindowHandle + 'surface>(
        config: surface::SurfaceConfig<W>,
        instance: &wgpu::Instance,
    ) -> surface::Result<Self> {
        let wgpu_surface = instance
            .create_surface(config.raw_window_handle)
            .map_err(|_| surface::SurfaceError::BackendError)?;
        Ok(Self { wgpu_surface })
    }
}
