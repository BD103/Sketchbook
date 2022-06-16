mod player;

pub use self::player::Player;

use crate::states::GameState;
use bevy::prelude::*;

pub struct LevelScreenPlugin;

impl Plugin for LevelScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Level).with_system(spawn_player))
            .add_system_set(SystemSet::on_exit(GameState::Level).with_system(despawn_player));
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/player/player.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0).with_scale(Vec3::new(5.0, 5.0, 0.0)),
            ..default()
        })
        .insert(Player)
        .insert(Name::new("Player"));
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for player in query.iter() {
        // If the player ever gets children, make recursive.
        commands.entity(player).despawn();
    }
}
