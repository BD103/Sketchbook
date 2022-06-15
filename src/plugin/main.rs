use crate::*;
use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Set initial game-state to GameState::Title
        app.add_state(GameState::Level)
            .insert_resource(WindowDescriptor {
                title: "Sketchbook".to_string(),
                resizable: false,
                ..default()
            })
            .insert_resource(ClearColor(palette::MONO.0))
            .add_startup_system(system::setup);
    }
}
