use crate::{
    palette,
    states::{GameState, LevelState},
};
use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::default())
            .add_state(LevelState::default())
            .insert_resource(WindowDescriptor {
                title: "Sketchbook".to_string(),
                resizable: false,
                ..default()
            })
            .insert_resource(ClearColor(palette::MONO.0))
            .add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Name::new("Main Camera"));
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Name::new("UI Camera"));

    info!("Finished initializing.");
}

#[allow(dead_code)]
pub fn hello_world() {
    println!("Hello, world!");
}
