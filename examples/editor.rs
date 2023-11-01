#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use frontier_folly::object::ObjectPlugins;
use space_editor::prelude::*;
use space_editor::ext::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: bevy::asset::ChangeWatcher::with_delay(std::time::Duration::from_millis(50)),
            ..Default::default()
        }))
        .add_plugins(SpaceEditorPlugin::default())
        .add_plugins(ObjectPlugins)
        .add_systems(Startup, space_enviroment)
        .insert_resource(Msaa::Off)
        .run();
}

fn space_enviroment(mut commands: Commands) {
    commands.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4096 });
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance : 5000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config: CascadeShadowConfigBuilder::default().into(),
        ..default()
    });

    //grid
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..default()
        },
        ..default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera : Camera {
                hdr : true,
                ..default()
            },
            camera_3d : Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
                ..default()
            },
            ..default()
        })
        .insert(BloomSettings::default())
        .insert(bevy::pbr::ScreenSpaceAmbientOcclusionBundle::default())
        .insert(PanOrbitCamera::default())
        .insert(RaycastPickCamera::default())
        .insert(EditorCameraMarker);
}
