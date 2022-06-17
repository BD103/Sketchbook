use crate::screens::level_screen::{Gravity, Player};
use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_system(exit_on_esc_system)
            .register_inspectable::<Player>()
            .register_inspectable::<Gravity>();
    }
}
