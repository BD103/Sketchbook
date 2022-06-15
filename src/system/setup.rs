use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Name::new("Main Camera"));
    commands.spawn_bundle(UiCameraBundle::default()).insert(Name::new("UI Camera"));
    info!("Finished initializing.");
}
