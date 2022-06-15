pub mod level_screen;
pub mod title_screen;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(title_screen::TitleScreenPlugin)
            .add(level_screen::LevelScreenPlugin);
    }
}
