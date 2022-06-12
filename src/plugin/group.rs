use crate::*;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct SketchbookPlugins;

impl PluginGroup for SketchbookPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(plugin::MainPlugin);
    }
}
