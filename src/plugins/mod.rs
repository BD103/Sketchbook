//! Contains plugins that cannot be sorted into separate categories.

pub mod dev;
pub mod main;

pub use self::{dev::DevPlugin, main::MainPlugin};
