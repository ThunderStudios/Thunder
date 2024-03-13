//! # thndr_window
//!
//! `thndr_window` is a simple windowing library for Rust, built for the Thunder Engine.
//!

use std::sync::{Arc, Mutex};

use anyhow::Result;
use thndr_app::prelude::*;
use thndr_ecs::prelude::*;
use thndr_event::prelude::*;
use thndr_math::prelude::*;
use thndr_tracing::prelude::*;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event as WinitEvent, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window as WinitWindow, WindowBuilder},
};

use input::prelude::*;

/// Provides input functionality.
pub mod input;

/// A component that represents a window configuration.
/// If the [WindowPlugin] is added to the [App],  the first entity with a [WindowConfig] component will be used to create the [Window].
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// The title of the window.
    pub title: String,
    /// The width of the window. In pixels.
    pub width: u32,
    /// The height of the window. In pixels.
    pub height: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "App".to_string(),
            width: 1280,
            height: 720,
        }
    }
}

/// A window resize event.
#[derive(Debug, Default, Clone)]
pub struct WindowResizeEvent {
    /// The new width of the window.
    pub width: u32,
    /// The new height of the window.
    pub height: u32,
}

/// The window [ScheduleRunner] for the [App].
#[derive(Default, Debug)]
pub struct WindowRunner(Arc<Mutex<Option<EventLoop<()>>>>);

impl ScheduleRunner for WindowRunner {
    fn run_app(&mut self, app: &mut App) {
        let mut startup = app.startup.build();
        let mut pre_update = app.pre_update.build();
        let mut update = app.update.build();
        let mut post_update = app.post_update.build();

        for system in &app.direct_startup {
            system(&mut app.world);
        }
        startup.execute((&mut app.world,)).expect("Startup failed");

        // take the event loop, so we can run it.
        let event_loop = std::mem::take(&mut *self.0.lock().unwrap()).unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop
            .run(move |event, elwt| match event {
                WinitEvent::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    debug!("Window closed!");
                    elwt.exit();
                }
                WinitEvent::AboutToWait => {
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
                WinitEvent::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {}
                WinitEvent::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    app.world.spawn((
                        Event::default(),
                        WindowResizeEvent {
                            width: size.width,
                            height: size.height,
                        },
                    ));
                }
                WinitEvent::WindowEvent {
                    event: WindowEvent::KeyboardInput { event, .. },
                    ..
                } => {
                    if event.state == ElementState::Pressed {
                        if let PhysicalKey::Code(code) = event.physical_key {
                            app.world
                                .spawn((Event::default(), KeyPressEvent { key: code }));
                        }
                    } else if let PhysicalKey::Code(code) = event.physical_key {
                        app.world
                            .spawn((Event::default(), KeyReleaseEvent { key: code }));
                    }
                }
                WinitEvent::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    app.world.spawn((
                        Event::default(),
                        MouseMoveEvent {
                            position: Vec2::new(position.x as f32, position.y as f32),
                        },
                    ));
                }
                WinitEvent::WindowEvent {
                    event: WindowEvent::MouseInput { state, button, .. },
                    ..
                } => {
                    if state == ElementState::Pressed {
                        app.world
                            .spawn((Event::default(), MousePressEvent { button }));
                    } else {
                        app.world
                            .spawn((Event::default(), MouseReleaseEvent { button }));
                    }
                }
                WinitEvent::WindowEvent {
                    event: WindowEvent::MouseWheel { delta, .. },
                    ..
                } => {
                    app.world.spawn((
                        Event::default(),
                        MouseScrollEvent {
                            delta: match delta {
                                // TODO: make this a bit more accurate
                                MouseScrollDelta::LineDelta(x, y) => Vec2::new(x as f32, y as f32),
                                MouseScrollDelta::PixelDelta(pos) => {
                                    Vec2::new(pos.x as f32, pos.y as f32)
                                }
                            },
                        },
                    ));
                }
                _ => {}
            })
            .expect("Failed to run event loop");
    }
}

/// A component that represents a window.
#[derive(Debug)]
pub struct Window {
    /// The window handle. The reason this is an `Arc` is because it *could* be shared with the render plugin.
    pub handle: Arc<WinitWindow>,
}

impl Window {
    /// Creates a new window.
    pub fn new(event_loop: &EventLoop<()>, config: WindowConfig) -> Result<Self> {
        let handle = WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build(event_loop)?;

        Ok(Self {
            handle: Arc::new(handle),
        })
    }
}

/// This tag marks the primary window.
#[derive(Debug, Default)]
pub struct PrimaryWindow;

/// The window plugin.
#[derive(Default, Debug)]
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        let event_loop = Arc::new(Mutex::new(Some(
            EventLoop::new().expect("Failed to create event loop"),
        )));

        app.set_runner(WindowRunner(event_loop.clone()));

        // spawn the primary window
        app.world.spawn((PrimaryWindow, WindowConfig::default()));

        // spawn the keys
        app.world.spawn((Keys::default(), Mouse::default())); // TODO: seperate input plugin

        app.add_direct_startup_system(move |world| {
            let configs = world
                .query::<&WindowConfig>()
                .without::<&Window>()
                .iter()
                .map(|(e, c)| (e, c.clone()))
                .collect::<Vec<_>>();

            for (entity, config) in configs {
                world
                    .insert_one(
                        entity,
                        Window::new(event_loop.lock().unwrap().as_ref().unwrap(), config)
                            .expect("Failed to create window"),
                    )
                    .unwrap();
            }
        });

        app.add_update_system(handle_input);
    }
}

/// System that registers input events with the [Mouse] and [Keys] components.
pub fn handle_input(
    world: SubWorld<(
        &mut Keys,
        &mut Mouse,
        &mut Event,
        &KeyPressEvent,
        &KeyReleaseEvent,
        &MouseMoveEvent,
        &MousePressEvent,
        &MouseReleaseEvent,
        &MouseScrollEvent,
    )>,
) {
    let mut keys = world.query::<&mut Keys>();
    let mut mouse = world.query::<&mut Mouse>();
    let mut key_press_events = world.query::<(&mut Event, &KeyPressEvent)>();
    let mut key_release_events = world.query::<(&mut Event, &KeyReleaseEvent)>();
    let mut mouse_move_events = world.query::<(&mut Event, &MouseMoveEvent)>();
    let mut mouse_press_events = world.query::<(&mut Event, &MousePressEvent)>();
    let mut mouse_release_events = world.query::<(&mut Event, &MouseReleaseEvent)>();
    let mut mouse_scroll_events = world.query::<(&mut Event, &MouseScrollEvent)>();

    for (_, keys) in keys.iter() {
        keys.update();
    }

    for (_, mouse) in mouse.iter() {
        mouse.update();
    }

    for (_, (e, key_press)) in key_press_events.iter() {
        if e.status != EventStatus::NotHandled {
            continue;
        }
        for (_, keys) in keys.iter() {
            keys.press(key_press.key);
            e.status = EventStatus::Handled;
        }
    }

    for (_, (e, key_release)) in key_release_events.iter() {
        if e.status != EventStatus::NotHandled {
            continue;
        }
        for (_, keys) in keys.iter() {
            keys.release(key_release.key);
            e.status = EventStatus::Handled;
        }
    }

    for (_, (e, mouse_move)) in mouse_move_events.iter() {
        if e.status != EventStatus::NotHandled {
            continue;
        }
        for (_, mouse) in mouse.iter() {
            mouse.move_to(mouse_move.position);
            e.status = EventStatus::Handled;
        }
    }

    for (_, (e, mouse_press)) in mouse_press_events.iter() {
        if e.status != EventStatus::NotHandled {
            continue;
        }
        for (_, mouse) in mouse.iter() {
            mouse.press(mouse_press.button);
            e.status = EventStatus::Handled;
        }
    }

    for (_, (e, mouse_release)) in mouse_release_events.iter() {
        if e.status != EventStatus::NotHandled {
            continue;
        }
        for (_, mouse) in mouse.iter() {
            mouse.release(mouse_release.button);
            e.status = EventStatus::Handled;
        }
    }

    for (_, (e, mouse_scroll)) in mouse_scroll_events.iter() {
        if e.status != EventStatus::NotHandled {
            continue;
        }
        for (_, mouse) in mouse.iter() {
            mouse.set_scroll(mouse_scroll.delta);
            e.status = EventStatus::Handled;
        }
    }
}

/// Common types, traits, and functions.
pub mod prelude {
    pub use super::{
        input::prelude::*, Window, WindowConfig, WindowPlugin, WindowResizeEvent, WindowRunner,
    };
}
