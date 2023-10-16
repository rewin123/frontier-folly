use bevy::{prelude::*, input::common_conditions::input_toggle_active};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use big_space::FloatingOrigin;
use frontier_folly::{controller::{ControllerPlugin, OrbitControler}, object::{small_hypergate::SmallHypergatePlugin, ship::Ship}, position::SpaceCell, enviroment::sand_cloud::{SandCloudSpawner, SandCloudPlugin}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugins(EguiPlugin)
        .add_plugins((
            big_space::FloatingOriginPlugin::<i64>::default(),
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)),
        )
        .add_plugins(ControllerPlugin)
        .add_plugins(SmallHypergatePlugin)
        .add_plugins(SandCloudPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands : Commands,
    assets : Res<AssetServer>
) {

    let ship = commands.spawn((SceneBundle {
            scene: assets.load("low_poly_fighter.glb#Scene0"),
            transform: Transform::default(),
            ..default()
        },
        SpaceCell::default(),
        Name::new("Ship"),
        Ship,
        SandCloudSpawner {
            ..default()
        }
    )).id();

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d : Camera3d {
                clear_color : bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            camera : Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        bevy::core_pipeline::bloom::BloomSettings {
            ..default()
        },
        SpaceCell::default(), // All spatial entities need this component
        FloatingOrigin, // Important: marks this as the entity to use as the floating origin
        OrbitControler {
            target: Some(ship),
            ..default()
        }
    ));

    // light
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            ..default()
        },
        transform : Transform::from_xyz(-5.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}