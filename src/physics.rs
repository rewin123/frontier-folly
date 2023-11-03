pub mod big_space_sync;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
pub struct PhysicsOrigin;  

pub struct SpacePhysicsPlugin;

impl Plugin for SpacePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            bevy_xpbd_3d::prelude::PhysicsPlugins::default()
                .build()
                .disable::<SyncPlugin>()
                .add(big_space_sync::SyncPlugin)
        );
    }
}