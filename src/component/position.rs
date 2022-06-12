use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Position(Vec2);
