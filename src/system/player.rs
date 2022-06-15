use crate::*;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprite/player/player.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0).with_scale(Vec3::new(5.0, 5.0, 0.0)),
            ..default()
        })
        .insert(entity_marker::Player)
        .insert(Name::new("Player"));
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<entity_marker::Player>>) {
    for player in query.iter() {
        // If the player ever gets children, make recursive
        commands.entity(player).despawn();
    }
}
