use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef}, asset::ChangeWatcher, core_pipeline::bloom::BloomSettings,
};
use frontier_folly::object::thruster_flame::FlameMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(std::time::Duration::from_millis(50)),
            ..Default::default()
        }))
        .add_plugins(MaterialPlugin::<FlameMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FlameMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: asset_server.load("flame.glb#Mesh0/Primitive0"),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(FlameMaterial {
            color: Color::ORANGE_RED,
        }),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
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
    .insert(BloomSettings::default());
}
