//! A basic example of how to create a hierarchy of entities in the ECS world.
use thndr::prelude::*;

fn setup(world: &mut World) {
    // the root entity
    let root = world.spawn((
        Position::new(0.0, 5.0, 0.0),
        Scale::new(1.0, 2.5, 3.9),
        "Root",
    ));

    // Create a loose entity
    let child = world.spawn((
        Position::new(3.0, 1.0, 0.0),
        Scale::new(1.8, 1.6, 1.3),
        "Child",
    ));

    // Create another loose entity
    let grand_child = world.spawn((Position::new(0.0, -1.0, 3.0), "Grand Child"));

    // Attaches the child to a parent, in this case `root`
    world.attach::<Tree>(child, root).unwrap();

    // Attaches the grand child to a parent, in this case `child`
    world.attach::<Tree>(grand_child, child).unwrap();
}

fn update(world: SubWorld<(&GlobalTransform, &&'static str)>) {
    let mut q = world.query::<(&GlobalTransform, &&str)>();

    for (entity, (global_transform, name)) in q.iter() {
        if *name == "Grand Child" {
            // now, the grand child should have a transform relative to the root entity
            println!(
                "Entity {:?} is at transform {:?}",
                entity,
                global_transform.to_scale_rotation_translation()
            );
        }
    }
}

fn main() {
    App::new()
        .add_plugin(TransformPlugin)
        // we use a direct system here, as we need full access to the world (using commands we would not get the entity id, which we need to attach entities to each other)
        .add_direct_startup_system(setup) // a direct system simply gets full access to the world, with no restrictions
        .add_update_system(update)
        .run();
}
