//! # thndr_gpu
//!
//! `thndr_gpu` is Thunder's GPU abstraction layer. It provides a simple and easy to use API for
//! working with the GPU.
//!
//! The plan is to support multiple backends, staring with Vulkan.
//!

/// Defines what backend is being used for the GPU.
pub mod backend;

/// Defines the abstract interface for the GPU.
/// We are NOT going to use traits, instead we will use an enum for static dispatch.
/// The reason for this is that the rust compiler can inline those calls,
/// which is really good for performance.
pub mod interface;
