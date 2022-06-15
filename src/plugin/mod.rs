mod dev;
mod level_screen;
mod main;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct SketchbookPlugins;

impl PluginGroup for SketchbookPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(self::main::MainPlugin);
        DefaultPlugins.build(group);

        group.add(self::level_screen::LevelScreenPlugin);

        #[cfg(debug_assertions)]
        group.add(self::dev::DevPlugin);
    }
}
