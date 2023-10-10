#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{prelude::*, window::exit_on_primary_closed, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use big_space::*;

type SpaceCell = GridCell<i64>;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugins((
            big_space::FloatingOriginPlugin::<i64>::default(),
            // big_space::camera::CameraControllerPlugin::<i64>::default(),
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets : Res<AssetServer>
) {

    let sun_size = 695500.0;

    let planet_1_size = 2000.0;
    let planet_1_dist = 1000000.0;
    

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(58000000.0 + 4000000.0 + 2439000.0, 1100000.0, 2439000.0 / 2.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 1e-16,
                ..default()
            }),
            camera_3d : Camera3d {
                clear_color : bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        SpaceCell::default(), // All spatial entities need this component
        FloatingOrigin, // Important: marks this as the entity to use as the floating origin
        camera::CameraController::default() // Built-in camera controller
            .with_max_speed(1000.0)
            .with_smoothness(0.95, 0.9)
            .with_speed(1.5),
    ));

    let get_rock_mat = |materials : &mut Assets<StandardMaterial>, idx : usize| {
        let mat = materials.add(StandardMaterial {
            perceptual_roughness: 0.8,
            reflectance: 0.5,
            base_color_texture : Some(assets.load(&format!("planets/Rock-EQUIRECTANGULAR-{}-1024x512.png", idx))),
            ..default()
        });
        mat
    };

    let get_planet_mat = |materials: &mut Assets<StandardMaterial>, tex_path : &str| {
        let mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            base_color_texture : Some(assets.load(tex_path)),
            ..default()
        });
        mat
    };

    let spawn_planet = |commands : &mut Commands, meshes : &mut Assets<Mesh>, radius : f32, sun_distance : f32, mat : Handle<StandardMaterial>| {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::UVSphere {
                    radius: 0.5,
                    sectors : 128,
                    stacks : 128,
                    ..default()
                }
                .into()),
                material: mat,
                transform: Transform::from_xyz(sun_distance, 0.0, 0.0).with_scale(Vec3::splat(radius)),
                ..default()
            }, 
            SpaceCell::default(),
        )).id()
    };

    //spawn star
    let sun = commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::UVSphere {
                    radius: 0.5,
                    ..default()
                }
                .into()),
                material: materials.add(StandardMaterial {
                    emissive: Color::rgb(1.0, 1.0, 1.0),
                    base_color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(6955000.0)),
                ..default()
            },
            SpaceCell::default(),
        )
    ).id();

    //spawn planets
    let first_planet = spawn_planet(
        &mut commands,
        &mut meshes,
        2439000.0, 
        58000000.0, 
        get_planet_mat(&mut materials, "planets/Volcanic-EQUIRECTANGULAR-1-1024x512.png"));

    let first_planet_moon = spawn_planet(
        &mut commands,
        &mut meshes,
        2439000.0  / 2.0, 
        58000000.0 + 2439000.0, 
        get_rock_mat(&mut materials, 1));



    // light
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            ..default()
        },
        ..default()
    },));
}