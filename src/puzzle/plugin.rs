use super::player;
use crate::states::GameState;
use bevy::prelude::*;

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Level).with_system(player::spawn_player))
            .add_system_set(
                SystemSet::on_exit(GameState::Level).with_system(player::despawn_player),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Level)
                    .with_system(player::update_player_input)
                    .with_system(player::rotate_player),
            );
    }
}
