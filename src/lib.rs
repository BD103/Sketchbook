mod palette;
mod plugins;
mod screens;
mod states;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct SketchbookPlugins;

impl PluginGroup for SketchbookPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        DefaultPlugins.build(group);

        group.add(plugins::MainPlugin);

        screens::ScreenPlugins.build(group);

        #[cfg(debug_assertions)]
        group.add(plugins::DevPlugin);
    }
}
