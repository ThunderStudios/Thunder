//! # thndr_app
//!
//! `thndr_app` is the application library for the Thunder Engine.
//! It defines the main application struct and the main loop.
//!

use std::sync::Arc;

use plugin::Plugin;
use thndr_ecs::prelude::*;

/// A plugin is a way to add functionality to the application.
pub mod plugin;

/// A runner for the application.
/// Every application needs a runner to run the schedules and systems.
/// The one most people will use is [thndr_window::WindowRunner].
pub trait ScheduleRunner {
    /// Runs the application.
    fn run_app(&mut self, app: &mut App);
}

/// The default runner for the application.
/// This runner is used when no other runner is specified.
#[derive(Default, Debug)]
pub struct DefaultRunner;

impl ScheduleRunner for DefaultRunner {
    fn run_app(&mut self, app: &mut App) {
        let mut startup = app.startup.build();
        let mut pre_update = app.pre_update.build();
        let mut update = app.update.build();
        let mut post_update = app.post_update.build();

        for system in &app.direct_startup {
            system(&mut app.world);
        }
        startup.execute((&mut app.world,)).expect("Startup failed");

        loop {
            for system in &app.direct_pre_update {
                system(&mut app.world);
            }
            pre_update
                .execute((&mut app.world,))
                .expect("Pre-update failed");

            for system in &app.direct_update {
                system(&mut app.world);
            }
            update.execute((&mut app.world,)).expect("Update failed");

            for system in &app.direct_post_update {
                system(&mut app.world);
            }
            post_update
                .execute((&mut app.world,))
                .expect("Post-update failed");
        }
    }
}

/// The main application struct.
///
/// This struct is the main entry point for the application.
#[derive(Default)]
pub struct App {
    /// The ECS world.
    pub world: World,
    /// The startup schedule.
    pub startup: ScheduleBuilder,
    /// The direct startup systems.
    pub direct_startup: Vec<Box<dyn Fn(&mut World)>>,
    /// The pre-update schedule.
    pub pre_update: ScheduleBuilder,
    /// The direct pre-update systems.
    pub direct_pre_update: Vec<Box<dyn Fn(&mut World)>>,
    /// The update schedule.
    pub update: ScheduleBuilder,
    /// The direct update systems.
    pub direct_update: Vec<Box<dyn Fn(&mut World)>>,
    /// The post-update schedule.
    pub post_update: ScheduleBuilder,
    /// The direct post-update systems.
    pub direct_post_update: Vec<Box<dyn Fn(&mut World)>>,
    /// The runner for the application.
    pub runner: Option<Box<dyn ScheduleRunner>>,
    /// The plugins for the application.
    pub plugins: Vec<Arc<dyn Plugin>>,
}

impl App {
    /// Create a new application.
    pub fn new() -> Self {
        Self {
            world: World::new(),
            startup: ScheduleBuilder::new(),
            pre_update: ScheduleBuilder::new(),
            update: ScheduleBuilder::new(),
            post_update: ScheduleBuilder::new(),
            direct_startup: Vec::new(),
            direct_pre_update: Vec::new(),
            direct_update: Vec::new(),
            direct_post_update: Vec::new(),
            runner: None,
            plugins: Vec::new(),
        }
    }

    /// Add a plugin to the application.
    pub fn add_plugin<P>(&mut self, plugin: P) -> &mut Self
    where
        P: 'static + Plugin,
    {
        self.plugins.push(Arc::new(plugin));

        self
    }

    /// Add a system to the startup schedule.
    pub fn add_startup_system<Args, Ret, S>(&mut self, system: S) -> &mut Self
    where
        S: 'static + System<Args, Ret> + Send,
    {
        self.startup.add_system(system);

        self
    }

    /// Add a system to the pre-update schedule.
    pub fn add_pre_update_system<Args, Ret, S>(&mut self, system: S) -> &mut Self
    where
        S: 'static + System<Args, Ret> + Send,
    {
        self.pre_update.add_system(system);

        self
    }

    /// Add a system to the update schedule.
    pub fn add_update_system<Args, Ret, S>(&mut self, system: S) -> &mut Self
    where
        S: 'static + System<Args, Ret> + Send,
    {
        self.update.add_system(system);

        self
    }

    /// Add a system to the post-update schedule.
    pub fn add_post_update_system<Args, Ret, S>(&mut self, system: S) -> &mut Self
    where
        S: 'static + System<Args, Ret> + Send,
    {
        self.post_update.add_system(system);

        self
    }

    /// Add a direct system to the startup schedule.
    pub fn add_direct_startup_system<F>(&mut self, system: F) -> &mut Self
    where
        F: 'static + Fn(&mut World),
    {
        self.direct_startup.push(Box::new(system));

        self
    }

    /// Add a direct system to the pre-update schedule.
    pub fn add_direct_pre_update_system<F>(&mut self, system: F) -> &mut Self
    where
        F: 'static + Fn(&mut World),
    {
        self.direct_pre_update.push(Box::new(system));

        self
    }

    /// Add a direct system to the update schedule.
    pub fn add_direct_update_system<F>(&mut self, system: F) -> &mut Self
    where
        F: 'static + Fn(&mut World),
    {
        self.direct_update.push(Box::new(system));

        self
    }

    /// Add a direct system to the post-update schedule.
    pub fn add_direct_post_update_system<F>(&mut self, system: F) -> &mut Self
    where
        F: 'static + Fn(&mut World),
    {
        self.direct_post_update.push(Box::new(system));

        self
    }

    /// Set the runner for the application.
    pub fn set_runner<R>(&mut self, runner: R) -> &mut Self
    where
        R: 'static + ScheduleRunner,
    {
        self.runner = Some(Box::new(runner));

        self
    }

    /// Run the application.
    pub fn run(&mut self) {
        let plugins = self.plugins.clone();
        for plugin in plugins {
            plugin.build(self);
        }

        let mut runner = self
            .runner
            .take()
            .unwrap_or_else(|| Box::new(DefaultRunner));

        runner.run_app(self);
    }
}

/// Common types, traits, and functions.
pub mod prelude {
    pub use crate::plugin::Plugin;
    pub use crate::{App, ScheduleRunner};
}
