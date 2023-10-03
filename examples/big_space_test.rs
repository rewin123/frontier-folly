#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use big_space::*;

type SpaceCell = GridCell<i32>;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugins((
            big_space::FloatingOriginPlugin::<i32>::default(),
            big_space::camera::CameraControllerPlugin::<i32>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50000.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 1e-16,
                ..default()
            }),
            ..default()
        },
        SpaceCell::default(), // All spatial entities need this component
        FloatingOrigin, // Important: marks this as the entity to use as the floating origin
        camera::CameraController::default() // Built-in camera controller
            .with_max_speed(1000.0)
            .with_smoothness(0.95, 0.9)
            .with_speed(1.5),
    ));

    let mesh_handle = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 32,
        }
        .try_into()
        .unwrap(),
    );
    let matl_handle = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        perceptual_roughness: 0.8,
        reflectance: 1.0,
        ..default()
    });

    //spawn planet
    commands.spawn((
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: matl_handle.clone(),
            transform: Transform::from_xyz(10000.0, 0.0, 0.0).with_scale(Vec3::splat(5000.0)),
            ..default()
        }, 
        SpaceCell::default()));

    //spawn moon
    commands.spawn((
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: matl_handle.clone(),
            transform: Transform::from_xyz(-5000.0, 0.0, 0.0).with_scale(Vec3::splat(1000.0)),
            ..default()
        },
        SpaceCell::default()));

    // light
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            ..default()
        },
        ..default()
    },));
}