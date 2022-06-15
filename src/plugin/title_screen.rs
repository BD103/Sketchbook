use crate::*;
use bevy::prelude::*;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Title).with_system(system::spawn_title_screen));
        app.add_system_set(SystemSet::on_exit(GameState::Title).with_system(system::despawn_title_screen));
    }
}
