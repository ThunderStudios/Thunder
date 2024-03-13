//! # thndr_event
//!
//! `thndr_event` is a simple event system for Rust, built for the Thunder Engine.
//!
//! For now, it simply works with closures, but in the future it will be extended to work with traits.
//!

use thndr_app::prelude::*;
use thndr_ecs::prelude::*;

// most cursed temporary fix,
// this works since everything in hecs is re-exported from thndr_ecs
extern crate self as hecs;

/// The status of an [Event].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventStatus {
    /// The event was not handled.
    #[default]
    NotHandled,
    /// The event was handled.
    Handled,
    /// The event was consumed.
    Consumed,
}

/// The [Event] type. Used to represent an event.
#[derive(Default, Debug, Clone)]
pub struct Event {
    /// The status of the event.
    pub status: EventStatus,
}

/// A system that handles event cleanup.
pub fn event_cleanup(world: SubWorld<&mut Event>, mut commands: Commands) {
    for (entity, event) in world.query::<&mut Event>().iter() {
        match event.status {
            EventStatus::NotHandled => event.status = EventStatus::Handled,
            EventStatus::Handled => event.status = EventStatus::Consumed,
            EventStatus::Consumed => commands.despawn(entity),
        }
    }
}

/// A plugin that handles event cleanup. At the end of every frame,
/// this will make all `NotHandled` events `Handled`.
/// and all `Handled` events `Consumed`.
/// and delete all `Consumed` events.
pub struct EventCleanupPlugin;

impl Plugin for EventCleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_post_update_system(event_cleanup);
    }
}

/// A bundle that contains an [Event] and a payload.
#[derive(Debug, Default, Clone, Bundle, DynamicBundleClone)]
pub struct EventBundle<T: Component> {
    /// The event.
    pub event: Event,
    /// The payload.
    pub payload: T,
}

/// Common types, traits, and functions.
pub mod prelude {
    pub use crate::{event_cleanup, Event, EventBundle, EventCleanupPlugin, EventStatus};
}
