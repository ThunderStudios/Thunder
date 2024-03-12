//! # thndr_transform
//!
//! `thndr_transform` is a collection of transform utilities for the Thunder game engine.
use std::ops::{Deref, DerefMut};

use thndr_math::prelude::*;

/// A Position in 3D space.
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

/// A Rotation in 3D space.
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

/// A Scale in 3D space.
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

/// Common transform types and functions.
pub mod prelude {
    pub use super::{GlobalTransform, Position, Rotation, Scale};
}
