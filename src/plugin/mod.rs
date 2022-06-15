mod dev;
mod level_screen;
mod main;
mod title_screen;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct SketchbookPlugins;

impl PluginGroup for SketchbookPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(self::main::MainPlugin);
        DefaultPlugins.build(group);

        group.add(self::title_screen::TitleScreenPlugin);
        group.add(self::level_screen::LevelScreenPlugin);

        #[cfg(debug_assertions)]
        group.add(self::dev::DevPlugin);
    }
}
