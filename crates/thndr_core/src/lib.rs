//! # thndr_core
//!
//! `thndr_core` is the core library for the Thunder Engine.
//! It's main job is to re-export all the other libraries in the engine.
//!

/// Re-exports all major libraries in the Thunder Engine.
pub mod prelude {
    pub use thndr_app::prelude::*;
    pub use thndr_ecs::prelude::*;
    pub use thndr_event::prelude::*;
    pub use thndr_math::prelude::*;
    pub use thndr_time::prelude::*;
    pub use thndr_tracing::prelude::*;
    pub use thndr_transform::prelude::*;
    pub use thndr_window::prelude::*;
}
