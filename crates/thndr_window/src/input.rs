use std::collections::HashSet;

use winit::{event::MouseButton, keyboard::KeyCode};

use thndr_math::prelude::*;

/// Component for easily checking key state.
#[derive(Debug, Default)]
pub struct Keys {
    /// The keys that are currently pressed.
    pressed: HashSet<KeyCode>,
    /// The keys that were just pressed.
    just_pressed: HashSet<KeyCode>,
    /// The keys that were just released.
    just_released: HashSet<KeyCode>,
}

impl Keys {
    /// Check if a key is pressed.
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    /// Check if a key was just pressed.
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    /// Check if a key was just released.
    pub fn just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }

    pub(crate) fn press(&mut self, key: KeyCode) {
        self.pressed.insert(key);
        self.just_pressed.insert(key);
    }

    pub(crate) fn release(&mut self, key: KeyCode) {
        self.pressed.remove(&key);
        self.just_released.insert(key);
    }

    pub(crate) fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

/// A key press event.
#[derive(Debug, Clone)]
pub struct KeyPressEvent {
    /// The key code of the key that was pressed.
    pub key: KeyCode,
}

/// A key release event.
#[derive(Debug, Clone)]
pub struct KeyReleaseEvent {
    /// The key code of the key that was released.
    pub key: KeyCode,
}

/// A component for easily checking mouse state.
#[derive(Debug, Default)]
pub struct Mouse {
    /// The position of the mouse.
    position: Vec2,
    /// The position of the mouse in the last frame.
    last_position: Vec2,
    /// The scroll delta of the mouse.
    scroll: Vec2,
    /// The buttons that are currently pressed.
    buttons: HashSet<MouseButton>,
    /// The buttons that were just pressed.
    just_pressed: HashSet<MouseButton>,
    /// The buttons that were just released.
    just_released: HashSet<MouseButton>,
}

impl Mouse {
    /// Get the position of the mouse.
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// Get the delta position of the mouse.
    pub fn delta(&self) -> Vec2 {
        self.position - self.last_position
    }

    /// Get the scroll delta of the mouse.
    pub fn scroll(&self) -> Vec2 {
        self.scroll
    }

    /// Check if a button is pressed.
    pub fn pressed(&self, button: MouseButton) -> bool {
        self.buttons.contains(&button)
    }

    /// Check if a button was just pressed.
    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed.contains(&button)
    }

    /// Check if a button was just released.
    pub fn just_released(&self, button: MouseButton) -> bool {
        self.just_released.contains(&button)
    }

    pub(crate) fn move_to(&mut self, position: Vec2) {
        self.last_position = self.position;
        self.position = position;
    }

    pub(crate) fn set_scroll(&mut self, scroll: Vec2) {
        self.scroll = scroll;
    }

    pub(crate) fn press(&mut self, button: MouseButton) {
        self.buttons.insert(button);
        self.just_pressed.insert(button);
    }

    pub(crate) fn release(&mut self, button: MouseButton) {
        self.buttons.remove(&button);
        self.just_released.insert(button);
    }

    pub(crate) fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
        self.scroll = Vec2::ZERO;
    }
}

/// A mouse move event.
#[derive(Default, Debug, Clone)]
pub struct MouseMoveEvent {
    /// The new position of the mouse.
    pub position: Vec2,
}

/// A mouse press event.
#[derive(Debug, Clone)]
pub struct MousePressEvent {
    /// The button that was pressed.
    pub button: MouseButton,
}

/// A mouse release event.
#[derive(Debug, Clone)]
pub struct MouseReleaseEvent {
    /// The button that was released.
    pub button: MouseButton,
}

/// A mouse scroll event.
#[derive(Debug, Clone)]
pub struct MouseScrollEvent {
    /// The scroll delta.
    pub delta: Vec2,
}

/// Common types, traits, and functions.
pub mod prelude {
    pub use super::{
        KeyPressEvent, KeyReleaseEvent, Keys, Mouse, MouseMoveEvent, MousePressEvent,
        MouseReleaseEvent, MouseScrollEvent,
    };

    pub use winit::{event::MouseButton, keyboard::KeyCode};
}
