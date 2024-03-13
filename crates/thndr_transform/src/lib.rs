//! # thndr_transform
//!
//! `thndr_transform` is a collection of transform utilities for the Thunder game engine.
use std::ops::{Deref, DerefMut};

use thndr_app::prelude::*;
use thndr_ecs::prelude::*;
use thndr_math::prelude::*;

/// A Position in 3D space. If in a [Tree], this is relative to the parent entity.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Position(pub Vec3);

impl Position {
    /// Creates a new position.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position(Vec3::new(x, y, z))
    }
}

impl Deref for Position {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec3> for Position {
    fn from(v: Vec3) -> Self {
        Position(v)
    }
}

impl From<Position> for Vec3 {
    fn from(p: Position) -> Self {
        p.0
    }
}

/// A Rotation in 3D space. If in a [Tree], this is relative to the parent entity.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Rotation(pub Quat);

impl Rotation {
    /// Creates a new rotation.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Rotation(Quat::from_xyzw(x, y, z, w))
    }
}

impl Deref for Rotation {
    type Target = Quat;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Quat> for Rotation {
    fn from(q: Quat) -> Self {
        Rotation(q)
    }
}

impl From<Rotation> for Quat {
    fn from(r: Rotation) -> Self {
        r.0
    }
}

/// A Scale in 3D space. If in a [Tree], this is relative to the parent entity.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scale(pub Vec3);

impl Default for Scale {
    fn default() -> Self {
        Scale(Vec3::ONE)
    }
}

impl Scale {
    /// Creates a new scale.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Scale(Vec3::new(x, y, z))
    }
}

impl Deref for Scale {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec3> for Scale {
    fn from(v: Vec3) -> Self {
        Scale(v)
    }
}

impl From<Scale> for Vec3 {
    fn from(s: Scale) -> Self {
        s.0
    }
}

/// A Transform in the global 3D space.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct GlobalTransform(pub Mat4);

impl GlobalTransform {
    /// Creates a new global transform.
    pub fn new(matrix: Mat4) -> Self {
        GlobalTransform(matrix)
    }

    /// Creates a new global transform from a position, rotation, and scale.
    pub fn from_position_rotation_scale(
        position: Position,
        rotation: Rotation,
        scale: Scale,
    ) -> Self {
        GlobalTransform(Mat4::from_scale_rotation_translation(
            scale.into(),
            rotation.into(),
            position.into(),
        ))
    }

    /// Creates a new global transform from a position and rotation.
    pub fn from_position_rotation(position: Position, rotation: Rotation) -> Self {
        GlobalTransform(Mat4::from_rotation_translation(
            rotation.into(),
            position.into(),
        ))
    }

    /// Creates a new global transform from a position and scale.
    pub fn from_position_scale(position: Position, scale: Scale) -> Self {
        GlobalTransform(*Self::from_position(position) * *Self::from_scale(scale))
    }

    /// Creates a new global transform from a rotation and scale.
    pub fn from_rotation_scale(rotation: Rotation, scale: Scale) -> Self {
        GlobalTransform(*Self::from_rotation(rotation) * *Self::from_scale(scale))
    }

    /// Creates a new global transform from a position.
    pub fn from_position(position: Position) -> Self {
        GlobalTransform(Mat4::from_translation(position.into()))
    }

    /// Creates a new global transform from a rotation.
    pub fn from_rotation(rotation: Rotation) -> Self {
        GlobalTransform(Mat4::from_quat(rotation.into()))
    }

    /// Creates a new global transform from a scale.
    pub fn from_scale(scale: Scale) -> Self {
        GlobalTransform(Mat4::from_scale(scale.into()))
    }
}

impl Deref for GlobalTransform {
    type Target = Mat4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Mat4> for GlobalTransform {
    fn from(m: Mat4) -> Self {
        GlobalTransform(m)
    }
}

impl From<GlobalTransform> for Mat4 {
    fn from(t: GlobalTransform) -> Self {
        t.0
    }
}

fn recurse_children(
    parent: Parent<Tree>,
    transform: GlobalTransform,
    world: &SubWorld<(
        &mut GlobalTransform,
        &Parent<Tree>,
        &Child<Tree>,
        &Position,
        &Rotation,
        &Scale,
    )>,
) {
    let mut to_recurse = Vec::new();
    {
        let mut q = world.query::<(
            &mut GlobalTransform,
            &Parent<Tree>,
            &Child<Tree>,
            &Position,
            &Rotation,
            &Scale,
        )>();

        let mut iter = q.iter();

        let last_child = iter.find_map(|(e, (g, pa, c, p, r, s))| {
            if e == parent.last_child() {
                Some((g, pa, c, p, r, s))
            } else {
                None
            }
        });

        if let Some(last_child) = last_child {
            let (global_transform, parent, child, position, rotation, scale) = last_child;

            // add the parent's transform, and use the position, rotation, and scale to calculate the new global transform
            *global_transform = GlobalTransform::new(
                *transform
                    * *GlobalTransform::from_position_rotation_scale(*position, *rotation, *scale),
            );

            let (global_transform, parent, child) = (*global_transform, *parent, *child);

            to_recurse.push((parent, global_transform));

            // do the other children
            let mut cur = iter.find_map(|(e, (g, pa, c, p, r, s))| {
                if e == child.prev() {
                    Some((g, *pa, *c, *p, *r, *s))
                } else {
                    None
                }
            });

            loop {
                if let Some((global_transform, parent, child, position, rotation, scale)) = cur {
                    *global_transform = GlobalTransform::new(
                        *transform
                            * *GlobalTransform::from_position_rotation_scale(
                                position, rotation, scale,
                            ),
                    );

                    let global_transform = *global_transform;

                    to_recurse.push((parent, global_transform));

                    cur = iter.find_map(|(e, (g, pa, c, p, r, s))| {
                        if e == child.prev() {
                            Some((g, *pa, *c, *p, *r, *s))
                        } else {
                            None
                        }
                    });
                } else {
                    break;
                }
            }
        }
    }

    // children that don't have children haven't been added yet
    {
        let mut q = world
            .query::<(
                &mut GlobalTransform,
                &Child<Tree>,
                &Position,
                &Rotation,
                &Scale,
            )>()
            .without::<&Parent<Tree>>();

        let mut iter = q.iter();

        let last_child = iter.find_map(|(e, (g, c, p, r, s))| {
            if e == parent.last_child() {
                Some((g, c, p, r, s))
            } else {
                None
            }
        });

        if let Some(last_child) = last_child {
            let (global_transform, child, position, rotation, scale) = last_child;

            *global_transform = GlobalTransform::new(
                *transform
                    * *GlobalTransform::from_position_rotation_scale(*position, *rotation, *scale),
            );

            let (_, child) = (*global_transform, *child);

            // no need to recurse, as there are no children
            // we do need to update the other children though

            let mut cur = iter.find_map(|(e, (g, c, p, r, s))| {
                if e == child.prev() {
                    Some((g, *c, *p, *r, *s))
                } else {
                    None
                }
            });

            loop {
                if let Some((global_transform, child, position, rotation, scale)) = cur {
                    *global_transform = GlobalTransform::new(
                        *transform
                            * *GlobalTransform::from_position_rotation_scale(
                                position, rotation, scale,
                            ),
                    );

                    cur = iter.find_map(|(e, (g, c, p, r, s))| {
                        if e == child.prev() {
                            Some((g, *c, *p, *r, *s))
                        } else {
                            None
                        }
                    });
                } else {
                    break;
                }
            }
        }
    }

    // recurse through the children
    to_recurse.par_iter().for_each(|(parent, transform)| {
        recurse_children(*parent, *transform, world);
    });
}

fn propagate_transforms(
    world: SubWorld<(
        &mut GlobalTransform,
        &Parent<Tree>,
        &Child<Tree>,
        &Position,
        &Rotation,
        &Scale,
    )>,
) {
    // first, we just set all GlobalTransforms based on the Position, Rotation and Scale.
    // afterwards, we ensure that if they are part of a Tree, they are transformed to the global space.
    // TODO: Properly handle gaps in the tree (where a parent doesnt have a transform, but a grandparent does)
    {
        let mut q = world.query::<(&mut GlobalTransform, &Position, &Rotation, &Scale)>();

        q.iter()
            .par_bridge()
            .for_each(|(_, (global_transform, position, rotation, scale))| {
                *global_transform =
                    GlobalTransform::from_position_rotation_scale(*position, *rotation, *scale);
            });
    }

    let mut q = world
        .query::<(&GlobalTransform, &Parent<Tree>)>()
        .without::<&Child<Tree>>();

    let iter = q.iter().map(|(_, (t, p))| (*t, *p)).collect::<Vec<_>>();

    drop(q); // prevent double borrow

    iter.par_iter().for_each(|(transform, parent)| {
        recurse_children(*parent, *transform, &world);
    });
}

fn insert_transforms(mut commands: Commands, world: SubWorld<(&Position, &Rotation, &Scale)>) {
    // every entity that doesn't have a global transform, but does have a position, rotation, OR scale, should get a global transform
    let mut q = world.query::<&Position>().without::<&GlobalTransform>();

    for (entity, _) in q.iter() {
        commands.insert_one(entity, GlobalTransform::default());
    }

    let mut q = world.query::<&Rotation>().without::<&GlobalTransform>();

    for (entity, _) in q.iter() {
        commands.insert_one(entity, GlobalTransform::default());
    }

    let mut q = world.query::<&Scale>().without::<&GlobalTransform>();

    for (entity, _) in q.iter() {
        commands.insert_one(entity, GlobalTransform::default());
    }

    // now ensure that they also have all 3 components
    let mut q = world.query::<()>().without::<&Position>();

    for (entity, _) in q.iter() {
        commands.insert_one(entity, Position::default());
    }

    let mut q = world.query::<()>().without::<&Rotation>();

    for (entity, _) in q.iter() {
        commands.insert_one(entity, Rotation::default());
    }

    let mut q = world.query::<()>().without::<&Scale>();

    for (entity, _) in q.iter() {
        commands.insert_one(entity, Scale::default());
    }
}

/// A plugin that simply updates the GlobalTransform component of all entities with a Position, Rotation, and Scale component.
/// Also translates from local to global space.
#[derive(Default, Debug)]
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_update_system(insert_transforms);
        app.add_post_update_system(propagate_transforms);
    }
}

/// Common transform types and functions.
pub mod prelude {
    pub use super::{GlobalTransform, Position, Rotation, Scale, TransformPlugin};
}
