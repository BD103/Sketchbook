//! Each unique entity type requires an entity marker like the following. This
//! allows entity types to be query-able.
//!
//! Each marker should be implemented like the following:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(Component)]
//! pub struct EntityName;
//! ```
//!
//! Additional derives are allowed. Custom `impls` are allowed as well, but
//! probably shouldn't be used as much.

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct TitleUIElement;
