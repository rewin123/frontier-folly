#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use space_editor::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceEditorPlugin::default())
        .add_systems(Startup, simple_editor_setup)
        .run();
}