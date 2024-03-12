//! # thndr_time
//!
//! `thndr_time` provides time related functionality, for the Thunder Game Engine.
//!

use std::{fmt, time::Instant};
use thndr_app::prelude::*;
use thndr_ecs::prelude::*;

/// A tracker for things like delta time, time since startup, etc.
#[derive(Debug)]
pub struct Time {
    startup: Instant,
    last_update: Instant,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    /// Creates a new [Time]
    pub fn new() -> Self {
        Self {
            startup: Instant::now(),
            last_update: Instant::now(),
        }
    }

    /// Returns the delta time since the last update. In f32 seconds.
    pub fn delta_seconds(&self) -> f32 {
        self.last_update.elapsed().as_secs_f32()
    }

    /// Returns the delta time since the last update. In f64 seconds.
    pub fn delta_seconds_f64(&self) -> f64 {
        self.last_update.elapsed().as_secs_f64()
    }

    /// Returns the time since the game started. In f32 seconds.
    pub fn seconds_since_startup(&self) -> f32 {
        self.startup.elapsed().as_secs_f32()
    }

    /// Returns the time since the game started. In f64 seconds.
    pub fn seconds_since_startup_f64(&self) -> f64 {
        self.startup.elapsed().as_secs_f64()
    }

    pub(crate) fn update(&mut self) {
        self.last_update = Instant::now();
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Time: {}", self.seconds_since_startup_f64())
    }
}

/// A plugin that simply keeps the time up to date.
#[derive(Default, Debug)]
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|mut commands: Commands| {
            commands.spawn((Time::new(),));
        });

        app.add_update_system(|world: SubWorld<&mut Time>| {
            for (_, time) in world.query::<&mut Time>().iter() {
                time.update();
            }
        });
    }
}
