#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::fmt::format;

use bevy::{prelude::*, window::{exit_on_primary_closed, PrimaryWindow}, input::common_conditions::input_toggle_active, utils::HashMap};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::*;
use big_space::*;

use frontier_folly::{controller::{DebugController, ControllerPlugin, OrbitControler}, object::small_hypergate::{SmallHypergatePlugin, CreateSmallHypergate}};
use serde::{Deserialize, Serialize};

type SpaceCell = GridCell<i64>;


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
        .add_event::<ControllerSwitch>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            debug_console,
            switch_to_ship,
            switch_to_ship_keymap
        ))
        .run();
}


#[derive(Component, Reflect)]
pub struct Ship;


#[derive(Event)]
enum ControllerSwitch {
    Orbit,
    Debug,
}

fn switch_to_ship_keymap(
    keys : Res<Input<KeyCode>>,
    mut events : EventWriter<ControllerSwitch>,
) {
    if keys.just_pressed(KeyCode::Space) {
        events.send(ControllerSwitch::Orbit);

    }
    if keys.just_pressed(KeyCode::D) {
        events.send(ControllerSwitch::Debug);
    }
}

fn switch_to_ship(
    mut commands: Commands,
    assets : Res<AssetServer>,
    query : Query<(Entity, &SpaceCell, &Transform), With<Camera>>,
    ship_query : Query<Entity, With<Ship>>,
    mut events : EventReader<ControllerSwitch>
) {
    for event in events.iter() {
        match event {
            ControllerSwitch::Orbit => {
                let (camera, camera_cell, camera_transform) = query.single();
                let ship = commands.spawn((SceneBundle {
                        scene: assets.load("low_poly_fighter.glb#Scene0"),
                        transform: camera_transform.clone(),
                        ..default()
                    },
                    camera_cell.clone(),
                    Name::new("Ship"),
                    Ship
                )).id();
                commands.entity(camera)
                    .remove::<DebugController>()
                    .remove::<Ship>()
                    .insert(OrbitControler {
                        target : Some(ship),
                        ..default()
                    }).insert(Transform::from_translation(camera_transform.translation + Vec3::splat(2.5)));
            },
            ControllerSwitch::Debug => {
                commands.entity(ship_query.single()).despawn_recursive();
                let (camera, camera_cell, camera_transform) = query.single();
                commands.entity(camera)
                    .remove::<OrbitControler>()
                    .insert((
                        DebugController::default(),
                        Ship
                    ));
            }
        }
    }
}

const GLOBAL_SCALE : f32 = 1.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets : Res<AssetServer>
) {
    let system = vec![
        CelestialBody {
            name : "Sun".to_string(),
            radius : 261_600_000.0 / GLOBAL_SCALE,
            distance : 0.0,    
            surface_texture : None,        
            children : vec![],
        },
        CelestialBody {
            name : "Moho".to_string(),
            radius: 250_000.0,
            distance : 5_263_138_304.0,
            surface_texture : Some(r#"planets\Volcanic-EQUIRECTANGULAR-1-1024x512.png"#.to_string()),
            children : vec![]
        },
        CelestialBody {
            name : "Eva".to_string(),
            radius : 700_000.0,
            distance : 9_832_684_544.0,
            surface_texture : Some(r#"planets\Primordial-Volcanic Clouds-EQUIRECTANGULAR-1-2048x1024.png"#.to_string()),
            children : vec![
                CelestialBody {
                    name : "Gilly".to_string(),
                    radius : 13_000.0,
                    distance : 31_500_000.0,
                    surface_texture : Some(r#"planets\Rock-EQUIRECTANGULAR-3-1024x512.png"#.to_string()),
                    children : vec![]
                }
            ]
        },
        CelestialBody {
            name : "Kerbal".to_string(),
            radius: 600_000.0,
            distance : 	13_599_840_256.0,
            surface_texture : Some(r#"planets\Oceanic-Clouds-EQUIRECTANGULAR-1-1024x512.png"#.to_string()),
            children : vec![
                CelestialBody {
                    name : "Mun".to_string(),
                    radius: 200_000.0,
                    distance : 12_000_000.0,
                    surface_texture : Some(r#"planets\Rock-EQUIRECTANGULAR-1-1024x512.png"#.to_string()),
                    children : vec![]
                },
                CelestialBody {
                    name : "Minimus".to_string(),
                    radius: 60_000.0,
                    distance: 47_000_000.0,
                    surface_texture : Some(r#"planets\Rock-EQUIRECTANGULAR-2-1024x512.png"#.to_string()),
                    children : vec![]
                }
            ]
        },
        CelestialBody {
            name : "Dune".to_string(),
            radius : 320_000.0,
            distance : 20_726_155_264.0,
            surface_texture : Some(r#"planets\Martian-EQUIRECTANGULAR-1-2048x1024.png"#.to_string()),
            children : vec![
                CelestialBody {
                    name : "Ike".to_string(),
                    radius : 130_000.0,
                    distance : 3_200_000.0,
                    surface_texture : Some(r#"planets\Rock-EQUIRECTANGULAR-4-1024x512.png"#.to_string()),
                    children : vec![]
                }
            ]
        }
    ]; 

    let mut poses : HashMap<String, Vec3> = HashMap::new();

    for celestial in system {
        poses.insert(celestial.name.clone(), Vec3::new(celestial.distance / GLOBAL_SCALE, 0.0, 0.0));
        spawn_celestial(
            &mut commands,
            &mut meshes,
            &mut materials,
            &assets,
            None,
            &celestial);
    }

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(poses["Kerbal"] + Vec3::new(12_000_000.0 , 0.0, 6_000_000.0 / GLOBAL_SCALE))
                .looking_at(poses["Kerbal"] + Vec3::new(0.0 , 0.0, 6_000_000.0 / GLOBAL_SCALE), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 1e-16,
                ..default()
            }),
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
        DebugController::default(),
        Ship
    ));

    // light
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            ..default()
        },
        ..default()
    },));
}

fn spawn_celestial(
    commands : &mut Commands,
    meshes : &mut Assets<Mesh>,
    materials : &mut Assets<StandardMaterial>,
    assets : &AssetServer,
    origin : Option<Vec3>,
    celestial : &CelestialBody
) {
    let mat = if let Some(path) = &celestial.surface_texture { 
        materials.add(StandardMaterial {
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            base_color_texture : Some(assets.load(path)),
            ..default()
        })
    } else {
        materials.add(StandardMaterial {
            emissive : Color::Rgba { red: 5.0, green: 5.0, blue: 1.0, alpha: 1.0 },
            base_color : Color::WHITE,
            ..default()
        })
    };

    let mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius: 1.0,
        sectors : 128,
        stacks : 128,
    }));

    let pos = if let Some(origin) = origin {
        origin + Vec3::new(0.0, 0.0, celestial.distance / GLOBAL_SCALE)
    } else {
        Vec3::new(celestial.distance / GLOBAL_SCALE, 0.0, 0.0)
    };

    commands.spawn((
        PbrBundle {
            mesh,
            material: mat,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(celestial.radius)),
            ..default()
        },
        SpaceCell::default(),
        Name::new(celestial.name.clone()),
        Celestial
    ));

    for child in &celestial.children {
        spawn_celestial(commands, meshes, materials, assets, Some(pos), child);
    }
}

fn debug_console(
    mut ctxs : Query<&mut EguiContext>,
    celestials : Query<(&SpaceCell, &Transform, &Name), With<Celestial>>,
    mut player : Query<(&mut SpaceCell, &mut Transform), (With<Ship>, Without<Celestial>)>,
    mut create_hypergate : EventWriter<CreateSmallHypergate>
) {
    egui::SidePanel::right("console").show(ctxs.single_mut().get_mut(), |ui| {
        let (mut player_grid, mut player_transform) = player.single_mut();

        if ui.button("Spawn hypergate").clicked() {
            create_hypergate.send(CreateSmallHypergate {
                spawn_cell: player_grid.clone(),
                spawn_transform: player_transform.clone(),
            });
        }

        for (grid, transform, name) in celestials.iter() {
            if ui.button(format!("Go to {}", name)).clicked() {
                *player_grid = *grid;

                player_transform.translation = transform.translation + Vec3::splat(transform.scale.x) * 2.0;
                player_transform.look_at(transform.translation, Vec3::Y);
            }    
        }
    });
}

#[derive(Component)]
struct Celestial;

#[derive(Serialize, Deserialize, Clone)]
struct CelestialBody {
    pub name : String,
    pub radius : f32,
    pub distance : f32,
    pub children : Vec<CelestialBody>,
    pub surface_texture : Option<String>
}