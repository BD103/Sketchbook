use crate::{palette, states::GameState};
use bevy::prelude::*;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Title).with_system(spawn_title_screen));
        app.add_system_set(SystemSet::on_exit(GameState::Title));
        app.add_system_set(SystemSet::on_update(GameState::Title));
    }
}

pub fn spawn_title_screen() {
    info!("Spawn title screen");
}
