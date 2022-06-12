use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vec2::new(0.0, 0.0))
    }
}
