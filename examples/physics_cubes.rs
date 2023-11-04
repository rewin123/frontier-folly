#![allow(clippy::unnecessary_cast)]

// code mostly from https://github.com/Jondolf/bevy_xpbd/blob/main/crates/bevy_xpbd_3d/examples/cubes.rs

use bevy::{prelude::*, math::DVec3, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::{math::*, prelude::*};

use big_space::FloatingOrigin;
use frontier_folly::{physics::{SpacePhysicsPlugin, PhysicsOrigin}, position::{SpacePositionPlugin, SpaceCell}};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.build().disable::<TransformPlugin>(), SpacePositionPlugin, SpacePhysicsPlugin))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

#[derive(Component)]
struct Cube;

/// The acceleration used for movement.
#[derive(Component)]
struct MovementAcceleration(Scalar);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Ground
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgb(0.7, 0.7, 0.8).into()),
            transform: Transform::from_xyz(0.0, -2.0, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(100.0, 1.0, 100.0),
        SpaceCell::default(),
        Position(Vector::NEG_Y * 2.0),
        PhysicsOrigin,
        Name::new("Ground"),
    ));

    let cube_size = 2.0;

    // Spawn cube stacks
    for x in -2..2 {
        for y in -2..2 {
            for z in -2..2 {
                let position = DVec3::new(x as f64, y as f64 + 5.0, z as f64) * (cube_size + 0.05);
                commands.spawn((
                    PbrBundle {
                        mesh: cube_mesh.clone(),
                        material: materials.add(Color::rgb(0.2, 0.7, 0.9).into()),
                        transform: Transform::from_translation(position.as_vec3())
                            .with_scale(Vec3::splat(cube_size as f32)),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(2.0, 2.0, 2.0),
                    Position(position + DVec3::Y * 5.0),
                    SpaceCell::default(),
                    Cube,
                ));
            }
        }
    }

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-1.0, -2.5, -1.5), Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 12.0, 40.0))
            .looking_at(Vec3::Y * 5.0, Vec3::Y),
        ..default()
    }).insert(SpaceCell::default())
        .insert(FloatingOrigin);
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Cube>>,
) {
    let k = 0.15;
    for mut lin_vel in &mut query {
        if keyboard_input.pressed(KeyCode::W) {
            lin_vel.z -= k;
        }
        if keyboard_input.pressed(KeyCode::S) {
            lin_vel.z += k;
        }
        if keyboard_input.pressed(KeyCode::A) {
            lin_vel.x -= k;
        }
        if keyboard_input.pressed(KeyCode::D) {
            lin_vel.x += k;
        }
    }
}