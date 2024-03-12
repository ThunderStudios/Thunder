//! # thndr_tracing
//!
//! `thndr_tracing` is a simple tracing library for Rust, built for the Thunder Engine.
//! It re-export the `tracing` crate, and a custom subscriber that is used by the engine.
//!

use thndr_app::prelude::*;
use tracing::Level;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

/// A plugin that initializes the tracing subscriber.
#[derive(Default, Debug)]
pub struct TracingPlugin;

impl Plugin for TracingPlugin {
    fn build(&self, _app: &mut App) {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(
                EnvFilter::builder()
                    .with_default_directive(Level::DEBUG.into())
                    .with_env_var("THNDR_LOG")
                    .from_env()
                    .expect("Failed to build filter"),
            )
            .init();
    }
}

/// Commonly used tracing types and functions.
pub mod prelude {
    pub use tracing;
    pub use tracing::{debug, error, info, instrument, trace, warn};
    pub use tracing::{debug_span, error_span, info_span, trace_span, warn_span};

    pub use crate::TracingPlugin;
}
