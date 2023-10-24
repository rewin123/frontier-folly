#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use space_editor::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceEditorPlugin::default())
        .add_systems(Startup, space_enviroment)
        .run();
}

fn space_enviroment(mut commands: Commands) {
    commands.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4096 });
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config: CascadeShadowConfigBuilder::default().into(),
        ..default()
    });

    //grid
    commands.spawn(bevy_infinite_grid::InfiniteGridBundle {
        grid: bevy_infinite_grid::InfiniteGrid {
            // shadow_color: None,
            ..default()
        },
        ..default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(bevy_panorbit_camera::PanOrbitCamera::default())
        .insert(bevy_mod_picking::prelude::RaycastPickCamera::default())
        .insert(EditorCameraMarker);
}
