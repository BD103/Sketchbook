use crate::*;
use bevy::prelude::*;

pub struct LevelScreenPlugin;

impl Plugin for LevelScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Level).with_system(system::spawn_player))
            .add_system_set(
                SystemSet::on_exit(GameState::Level).with_system(system::despawn_player),
            );
    }
}
