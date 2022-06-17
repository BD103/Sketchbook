mod player;

pub use self::player::*;

use crate::states::GameState;
use bevy::prelude::*;

pub struct LevelScreenPlugin;

impl Plugin for LevelScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Level).with_system(spawn_player))
            .add_system_set(SystemSet::on_exit(GameState::Level).with_system(despawn_player))
            .add_system_set(SystemSet::on_update(GameState::Level).with_system(update_player));
    }
}
