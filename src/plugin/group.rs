use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct SketchbookPlugins;

impl PluginGroup for SketchbookPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(super::main::MainPlugin);
        DefaultPlugins.build(group);

        #[cfg(debug_assertions)]
        group.add(super::dev::DevPlugin);
    }
}
