//! This example shows how to do basic input handling with Thunder.
use thndr::prelude::*;

fn check_keys(world: SubWorld<&Keys>) {
    let mut q = world.query::<&Keys>();
    let (_, keys) = q.iter().next().unwrap(); // there should be only one

    if keys.just_pressed(KeyCode::Space) {
        println!("Space was just pressed!");
    }
    if keys.just_released(KeyCode::Space) {
        println!("Space was just released!");
    }
    if keys.pressed(KeyCode::Space) {
        println!("Space is being held down!");
    }
}

fn check_mouse_movement(world: SubWorld<&Mouse>) {
    let mut q = world.query::<&Mouse>();
    let (_, mouse) = q.iter().next().unwrap(); // there should be only one

    println!("Mouse delta: {:?}", mouse.delta());
}

fn main() {
    App::new()
        .add_plugin(WindowPlugin) // Window plugin is required for input handling
        .add_update_system(check_keys)
        .add_update_system(check_mouse_movement)
        .run();
}
