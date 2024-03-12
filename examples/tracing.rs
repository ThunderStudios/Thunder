//! This example demonstrates how to use the [TracingPlugin] to log events in the application.
use thndr::prelude::*;

fn update() {
    debug!("Hello, Thunder!");
}

fn main() {
    App::new()
        .add_plugin(TracingPlugin)
        .add_update_system(update)
        .run();
}
