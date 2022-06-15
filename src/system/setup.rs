use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(Name::new("Camera"));
    info!("Finished initializing.");
}
