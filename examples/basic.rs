//! A basic example of how to use Thunder.
use thndr::prelude::*;

fn update() {
    println!("Hello, Thunder!");
}

fn main() {
    App::new().add_update_system(update).run();
}
