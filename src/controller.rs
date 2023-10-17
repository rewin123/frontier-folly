use bevy::prelude::*;

pub struct ControllerPlugin;

pub mod debug_controller;
pub mod orbit_controller;
pub mod fighter_controller;

pub use debug_controller::*;
pub use orbit_controller::*;
pub use fighter_controller::*;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DebugControllerPlugin,
            OrbitControllerPlugin,
            FighterControllerPlugin
        ));
    }   
}
