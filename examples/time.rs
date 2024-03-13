//! A basic example of how to use Thunder's time system.
use thndr::prelude::*;

fn update(world: SubWorld<&Time>) {
    for (_, time) in world.query::<&Time>().iter() {
        println!("Time: {}", time.delta_seconds());
    }
}

fn main() {
    App::new()
        .add_plugin(TimePlugin)
        .add_update_system(update)
        .run();
}
