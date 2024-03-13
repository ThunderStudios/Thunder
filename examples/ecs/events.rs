//! Shows some basic event handling.

use thndr::prelude::*;

/// A basic event containing a string.
#[derive(Debug, Default, Clone)]
struct MyEvent(String);

fn emit_event(mut commands: Commands) {
    commands.spawn(EventBundle {
        payload: MyEvent("Hello, World!".to_string()),
        ..Default::default()
    });
}

fn listener_system(events: SubWorld<(&mut Event, &MyEvent)>) {
    for (_, (status, event)) in events.query::<(&mut Event, &MyEvent)>().iter() {
        if status.status == EventStatus::NotHandled {
            println!("Received event: {:?}", event.0);
            status.status = EventStatus::Handled;
        }
    }
}

fn main() {
    App::new()
        .add_update_system(emit_event)
        .add_update_system(listener_system)
        .run();
}
