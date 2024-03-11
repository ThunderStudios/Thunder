//! # thndr_ecs
//!
//! `thndr_ecs` is a simple entity-component-system library for Rust, built for the Thunder Engine.
//!
//! Currently, it is a thin wrapper over [hecs](https://crates.io/crates/hecs), with some extra scheduling of systems.

use hecs::{Component, DynamicBundle, Entity};
use hecs_hierarchy::HierarchyMut;
use hecs_schedule::{CommandBuffer, Write};

/// The `Commands` type. Used to defer operations on the world.
pub type Commands<'a> = Write<'a, CommandBuffer>;

/// The `CommandsExt` trait. Extends the `Commands` type with additional functionality related to hierarchies.
pub trait CommandsExt {
    /// Attach `child` to `parent`. Parent does not require an existing `Parent component`.
    /// *Note*: The entity needs to be explicitly detached before being removed.
    fn attach<T: Component>(&mut self, child: Entity, parent: Entity);

    /// Attach a new entity with specified components to `parent`. Parent does not require an existing `Parent component`.
    fn attach_new<T: Component, C: DynamicBundle + Send + Sync + 'static>(
        &mut self,
        parent: Entity,
        components: C,
    );

    /// Detaches all children from entity and detaches entity from parent. Use this before removing
    /// entities to ensure no loose entity ids.
    fn detach_all<T: Component>(&mut self, entity: Entity);

    /// Detaches all children of parent.
    fn detach_children<T: Component>(&mut self, parent: Entity);
    /// Despawn all children of parent.
    fn despawn_children<T: Component>(&mut self, parent: Entity);

    /// Detach the child from tree `T`. The children of `child` will not remain in hierachy, but will
    /// remain attached to `child`, which means a later attach also will attach the children of `child`
    /// into the hierarchy. Essentially moving the subtree.
    fn detach<T: Component>(&mut self, child: Entity);

    /// Despawn parent and all children recursively. Essentially despawns a whole subtree including
    /// root. Does not fail if there are invalid, dangling IDs in tree.
    fn despawn_all<T: Component>(&mut self, parent: Entity);
}

impl CommandsExt for Commands<'_> {
    fn attach<T: Component>(&mut self, child: Entity, parent: Entity) {
        self.write(move |world| {
            world
                .attach::<T>(child, parent)
                .expect("Failed to attach entity");
        });
    }

    fn attach_new<T: Component, C: DynamicBundle + Send + Sync + 'static>(
        &mut self,
        parent: Entity,
        components: C,
    ) {
        self.write(move |world| {
            world
                .attach_new::<T, C>(parent, components)
                .expect("Failed to attach entity");
        });
    }

    fn detach_all<T: Component>(&mut self, entity: Entity) {
        self.write(move |world| {
            world
                .detach_all::<T>(entity)
                .expect("Failed to detach entity");
        });
    }

    fn detach_children<T: Component>(&mut self, parent: Entity) {
        self.write(move |world| {
            world
                .detach_children::<T>(parent)
                .expect("Failed to detach children");
        });
    }

    fn despawn_children<T: Component>(&mut self, parent: Entity) {
        self.write(move |world| {
            world
                .despawn_children::<T>(parent)
                .expect("Failed to despawn children");
        });
    }

    fn detach<T: Component>(&mut self, child: Entity) {
        self.write(move |world| {
            world.detach::<T>(child).expect("Failed to detach entity");
        });
    }

    fn despawn_all<T: Component>(&mut self, parent: Entity) {
        self.write(move |world| {
            world.despawn_all::<T>(parent);
        });
    }
}

/// Exports common types, traits, and functions.
pub mod prelude {
    pub use hecs::{Entity, World};
    pub use hecs_hierarchy::*;
    pub use hecs_schedule::*;
    pub use rayon::prelude::*;

    pub use crate::Commands;
    pub use crate::CommandsExt;
}
