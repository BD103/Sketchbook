use crate::*;
use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Title)
            .insert_resource(WindowDescriptor {
                title: "Sketchbook".to_string(),
                resizable: false,
                ..default()
            })
            .insert_resource(ClearColor(palette::MONO.0))
            .add_startup_system(system::setup);
    }
}
