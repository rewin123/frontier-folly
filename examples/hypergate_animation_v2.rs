use std::time::Duration;

use bevy::{prelude::*, core_pipeline::bloom::BloomSettings, input::keyboard::KeyboardInput, ecs::system::EntityCommands};
use bevy_egui::egui::Key;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::{lens::*, *};

const GATE_BUILDER_COUNT : usize = 6;
const GATE_RADIUS : f32 = 10.0;
const GATE_BUILDER_SPEED : f32 = 2.0;
const GATE_SPAWN_DIST : f32 = 10.0;
const GATE_CHARGE_TIME : f32 = 2.0;
const BEAM_COLOR : Color = Color::rgb(0.0, 10.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build())
        .add_systems(Startup, (
            camera_setup,
            setup_ship
        ))
        .add_systems(PostUpdate, spawn_hypergate)
        .add_systems(Update, (
            
            portal_edges,
        ))
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(TweeningPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

#[derive(Component)]
struct Hypergate {
    builders : Vec<Entity>,
    spawned : bool,
}


#[derive(Component)]
struct HypergateBuilder {
    angle : f32,
    neighbors : Vec<Entity>
}

fn portal_edges(
    mut gizmos : Gizmos,
    mut query : Query<(&GlobalTransform, &HypergateBuilder)>,
    charged : Query<&GlobalTransform>
) {
    for (transform, hypergate) in query.iter_mut() {
        for neighbor in hypergate.neighbors.iter() {
            if let Ok(neighbor_transform) = charged.get(*neighbor) {
                gizmos.line(
                    transform.translation(),
                    neighbor_transform.translation(),
                    BEAM_COLOR
                );
            }
        }
    }
}


fn spawn_hypergate(
    mut commands: Commands,
    input : Res<Input<KeyCode>>,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
) {
    if input.just_released(KeyCode::W) {
        let mesh = meshes.add(shape::Box::new(0.1, 0.1, 0.1).into());
        let mat = materials.add(StandardMaterial {
            base_color : Color::GRAY,
            ..default()
        });

        let gate = commands.spawn(SpatialBundle::default()).id();

        let base_dur = 5.0;
        let pre_open_dur = base_dur / 2.0;
        let rotate_speed = 1.0;

        //gate center animation
        let seq = Sequence::new([
            Tracks::new([
                Tween::new( //move forward
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(pre_open_dur),
                    TransformPositionLens {
                        start : Vec3::ZERO,
                        end : Vec3::new(0.0, 0.0, -GATE_SPAWN_DIST)
                    }
                ),
                Tween::new( //rotate
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(pre_open_dur),
                    TransformRotateAxisLens {
                        axis: Vec3::Z,
                        start: 0.0,
                        end: rotate_speed * std::f32::consts::PI * pre_open_dur,
                })
            ]),
            Tracks::new([Tween::new( //move backward
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(base_dur),
                    TransformPositionLens {
                        start : Vec3::new(0.0, 0.0, -GATE_SPAWN_DIST),
                        end : Vec3::new(0.0, 0.0, GATE_SPAWN_DIST)
                    }
                ),
                Tween::new( //rotate
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(base_dur),
                    TransformRotateAxisLens {
                        axis: Vec3::Z,
                        start: 0.0,
                        end: rotate_speed * std::f32::consts::PI * base_dur,
                })
            ])
        ]);

        let gate_position = Vec3::new(0.0, 0.0, 0.0);

        let mut entities = vec![];
        for _ in 0..GATE_BUILDER_COUNT {
            entities.push(commands.spawn_empty().id());
        }

        for i in 0..GATE_BUILDER_COUNT {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / GATE_BUILDER_COUNT as f32;
            let target = Vec3::new(gate_position.x + 0.2 * angle.sin(), gate_position.y  + 0.2 * angle.cos(), 0.0);

            let seq = Sequence::new(
                [
                    BoxedTweenable::from(bevy_tweening::Delay::new(Duration::from_secs_f32(pre_open_dur))),
                    BoxedTweenable::from(Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs_f32(base_dur / 2.0),
                        TransformPositionLens {
                            start : target.clone(),
                            end : target * GATE_RADIUS
                        }
                    )),
                    BoxedTweenable::from(Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs_f32(base_dur / 2.0),
                        TransformPositionLens {
                            start : target * GATE_RADIUS,
                            end : target.clone()
                        }
                    ))
                ]
            );

            commands
                .entity(entities[i])
                .insert((
                    PbrBundle {
                        transform : Transform::from_translation(target).looking_at(gate_position, Vec3::Z),
                        mesh : mesh.clone(),
                        material: mat.clone(),
                        ..default()
                    },
                    HypergateBuilder {
                        angle,
                        neighbors : vec![entities[(i + 1) % GATE_BUILDER_COUNT], entities[(i + GATE_BUILDER_COUNT - 1) % GATE_BUILDER_COUNT]],
                    },
                    Animator::new(seq)))
                .set_parent(gate);
        }

        commands.entity(gate).insert(Hypergate {
            builders : entities,
            spawned : false
        })
        .insert(Animator::new(seq));
    }
}


fn setup_ship(
    mut commands: Commands,
    assets : Res<AssetServer>,
) {
    commands.spawn(
        SceneBundle {
            scene: assets.load("low_poly_fighter.glb#Scene0"),
            ..default()
        });
}

fn camera_setup(
    mut commands: Commands,
) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        ..default()
    },
    BloomSettings::default()
    ));

    // light
    commands.spawn((DirectionalLightBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            ..default()
        },
        ..default()
    },));
}