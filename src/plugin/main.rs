use crate::*;
use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(system::setup);
    }
}
