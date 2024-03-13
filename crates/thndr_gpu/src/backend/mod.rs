/// The available backends for accessing the GPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
    /// Vulkan backend.
    Vulkan,
    #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
    /// Direct3D 11 backend.
    D3D11,
    #[cfg(all(feature = "d3d12", target_os = "windows", not(target_arch = "wasm32")))]
    /// Direct3D 12 backend.
    D3D12,
    // TODO: metal, webgpu
}

#[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
/// The Vulkan backend.
pub mod vulkan;

#[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
/// The Direct3D 11 backend.
pub mod d3d11;
