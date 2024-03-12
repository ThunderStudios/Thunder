//! This example demonstrates how to create an entity in the ECS world, and update it every frame.
use thndr::prelude::*;

// this system will get called once at the start of the app
// we receive a `Commands` type as an argument to the system.
// this type is used to add entities and components to the ECS world.
fn setup(mut commands: Commands) {
    // Every entity has zero or more `Component`s.
    // A component is a piece of data that is associated with an entity, but not stored in the entity itself.
    // Components are stored in a separate data structure called a "component storage".
    // This allows for efficient iteration over all entities with a given component.

    // In this example, we create an entity with a single component, `Position`.
    // The `Position` component is a simple 3D position in space.
    // You may notice that we specify the components as a tuple, even if there is only one component.
    // This is because the `Commands` type is generic over the types of components it can add.
    // This allows for adding multiple components at once, and is more efficient than adding them one at a time.
    commands.spawn((Position::new(0.0, 0.0, 0.0),));
}

// this system will get called every frame
// we specify something called a `SubWorld` as an argument to the system.
// this specifies that the system will only have access to the entities and components that are in the `SubWorld`.
// this is useful for performance, as it allows for the system to only access the data it needs,
// making it possible to run other systems in parallel.
fn update(sub_world: SubWorld<&Position>) {
    // we can iterate over all entities with a given component using the `query` method.
    // the `query` method returns an iterator over all entities with the given component, and a borrowed reference to the component data.
    for (entity, position) in sub_world.query::<&Position>().iter() {
        // we can access the component data using the borrowed reference.
        // in this case, we just print the position of the entity.
        println!("Entity {:?} is at position {:?}", entity, position);
    }
}

fn main() {
    // here we create a new app, and add the `startup` and `update` systems to it.
    // then will simply call run!
    App::new()
        .add_startup_system(setup) // add the `setup` system to the `startup` schedule
        .add_update_system(update) // add the `update` system to the `update` schedule
        .run();
}
