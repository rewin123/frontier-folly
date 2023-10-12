use bevy::{prelude::*, app::PluginGroupBuilder};

pub mod small_hypergate;

pub struct ObjectPlugins;

impl PluginGroup for ObjectPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(small_hypergate::SmallHypergatePlugin)
    }
}