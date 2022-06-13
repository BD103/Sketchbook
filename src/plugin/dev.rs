use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new());
    }
}
