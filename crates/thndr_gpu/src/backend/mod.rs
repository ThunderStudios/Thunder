/// The available backends for accessing the GPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    #[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
    /// Vulkan backend.
    Vulkan,
    /// The WebGPU backend.
    #[cfg(feature = "webgpu")]
    WebGPU,
    #[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
    /// Direct3D 11 backend.
    D3D11,
    // TODO: d3d12, metal
}

#[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
/// The Vulkan backend.
pub mod vulkan;

#[cfg(feature = "webgpu")]
/// The WebGPU backend.
pub mod webgpu;

#[cfg(all(feature = "d3d11", target_os = "windows", not(target_arch = "wasm32")))]
/// The Direct3D 11 backend.
pub mod d3d11;
