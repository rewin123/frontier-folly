use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

use crate::position::{SpaceCell, SpacePosition};

const GATE_BUILDER_COUNT: usize = 6;
const GATE_RADIUS: f32 = 10.0;
const GATE_SPAWN_DIST: f32 = 5.0;
const GATE_BASE_TIME: f32 = 2.5;
const BEAM_COLOR: Color = Color::rgb(217.0 / 255.0 * 10.0, 0.0, 91.0 / 255.0 * 10.0);

pub struct SmallHypergatePlugin;

impl Plugin for SmallHypergatePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }

        app.add_event::<CreateSmallHypergate>()
            .add_systems(PostUpdate, spawn_hypergate)
            .add_systems(
                PostUpdate,
                (
                    portal_edges.after(bevy::transform::TransformSystem::TransformPropagate),
                    small_hypergate_time_system,
                    small_hypergate_output,
                ),
            );
    }
}

#[derive(Component)]
struct HypergateOutput {
    pub start_time: Duration,
}

#[derive(Component)]
pub struct SmallHypergate {
    builders: Vec<Entity>,
    event_sended: bool,
    start_time: Duration,
    pub target_cell: SpaceCell,
    pub target_transform: Transform,
    pub opened: bool,
}

#[derive(Event)]
pub struct CreateSmallHypergate {
    pub spawn_cell: SpaceCell,
    pub spawn_transform: Transform,
    pub target_cell: SpaceCell,
    pub target_transform: Transform,
}

#[derive(Component)]
struct SmallHypergateBuilder {
    angle: f32,
    neighbors: Vec<Entity>,
}

fn portal_edges(
    mut gizmos: Gizmos,
    mut query: Query<(&GlobalTransform, &SmallHypergateBuilder)>,
    charged: Query<&GlobalTransform>,
) {
    for (transform, hypergate) in query.iter_mut() {
        for neighbor in hypergate.neighbors.iter() {
            if let Ok(neighbor_transform) = charged.get(*neighbor) {
                gizmos.line(
                    transform.translation(),
                    neighbor_transform.translation(),
                    BEAM_COLOR,
                );
            }
        }
    }
}

fn spawn_hypergate(
    mut commands: Commands,
    mut input: EventReader<CreateSmallHypergate>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    mut time: Res<Time>,
) {
    for params in input.iter() {
        //create parent for animation
        let parent = commands
            .spawn(big_space::FloatingSpatialBundle {
                transform: params.spawn_transform.clone(),
                grid_position: params.spawn_cell,
                ..default()
            })
            .id();

        let mesh = meshes.add(shape::Box::new(0.1, 0.1, 0.1).into());
        let mat = materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        });

        let gate = commands.spawn(SpatialBundle::default()).id();

        let base_dur = GATE_BASE_TIME;
        let pre_open_dur = base_dur / 2.0;
        let rotate_speed = 2.0;
        let pre_open_rotate = rotate_speed * std::f32::consts::PI * pre_open_dur;

        //gate center animation
        let seq = Sequence::new([
            Tracks::new([
                Tween::new(
                    //move forward
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(pre_open_dur),
                    TransformPositionLens {
                        start: Vec3::ZERO,
                        end: Vec3::new(0.0, 0.0, -GATE_SPAWN_DIST * 2.0),
                    },
                ),
                Tween::new(
                    //rotate
                    EaseFunction::QuadraticIn,
                    Duration::from_secs_f32(pre_open_dur),
                    TransformRotateAxisLens {
                        axis: Vec3::Z,
                        start: 0.0,
                        end: pre_open_rotate,
                    },
                ),
            ]),
            Tracks::new([
                Tween::new(
                    //move backward
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(base_dur),
                    TransformPositionLens {
                        start: Vec3::new(0.0, 0.0, -GATE_SPAWN_DIST * 2.0),
                        end: Vec3::new(0.0, 0.0, GATE_SPAWN_DIST),
                    },
                ),
                Tween::new(
                    //rotate
                    EaseFunction::QuadraticOut,
                    Duration::from_secs_f32(base_dur),
                    TransformRotateAxisLens {
                        axis: Vec3::Z,
                        start: pre_open_rotate,
                        end: pre_open_rotate + rotate_speed * std::f32::consts::PI * base_dur,
                    },
                ),
            ]),
        ]);

        let scale_seq = Sequence::new([
            Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(pre_open_dur),
                TransformScaleLens {
                    start: Vec3::splat(0.2),
                    end: Vec3::splat(GATE_RADIUS * 0.25 * 0.2),
                },
            ),
            Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(base_dur / 2.0),
                TransformScaleLens {
                    start: Vec3::splat(GATE_RADIUS * 0.25 * 0.2),
                    end: Vec3::splat(GATE_RADIUS * 0.2),
                },
            ),
            Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(base_dur / 2.0),
                TransformScaleLens {
                    start: Vec3::splat(GATE_RADIUS * 0.2),
                    end: Vec3::splat(0.2),
                },
            ),
        ]);

        let gate_position = Vec3::new(0.0, 0.0, 0.0);

        let mut entities = vec![];
        for _ in 0..GATE_BUILDER_COUNT {
            entities.push(commands.spawn_empty().id());
        }

        for i in 0..GATE_BUILDER_COUNT {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / GATE_BUILDER_COUNT as f32;
            let target = Vec3::new(
                gate_position.x + 0.2 * angle.sin(),
                gate_position.y + 0.2 * angle.cos(),
                0.0,
            );

            let seq = Sequence::new([
                BoxedTweenable::from(Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(pre_open_dur),
                    TransformPositionLens {
                        start: target.clone(),
                        end: target * GATE_RADIUS * 0.25,
                    },
                )),
                BoxedTweenable::from(Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(base_dur / 2.0),
                    TransformPositionLens {
                        start: target * GATE_RADIUS * 0.25,
                        end: target * GATE_RADIUS,
                    },
                )),
                BoxedTweenable::from(Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(base_dur / 2.0),
                    TransformPositionLens {
                        start: target * GATE_RADIUS,
                        end: target.clone(),
                    },
                )),
            ]);

            commands
                .entity(entities[i])
                .insert((
                    PbrBundle {
                        transform: Transform::from_translation(target)
                            .looking_at(gate_position, Vec3::Z),
                        mesh: mesh.clone(),
                        material: mat.clone(),
                        ..default()
                    },
                    SmallHypergateBuilder {
                        angle,
                        neighbors: vec![
                            entities[(i + 1) % GATE_BUILDER_COUNT],
                            entities[(i + GATE_BUILDER_COUNT - 1) % GATE_BUILDER_COUNT],
                        ],
                    },
                    Animator::new(seq),
                ))
                .set_parent(gate);
        }

        //spawn gate mesh
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(1.0, GATE_BUILDER_COUNT).into()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BLACK,
                        emissive: BEAM_COLOR,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, -0.01),
                    ..default()
                },
                Animator::new(scale_seq),
            ))
            .set_parent(gate);

        commands
            .entity(gate)
            .insert(SmallHypergate {
                builders: entities,
                event_sended: false,
                start_time: time.elapsed(),
                target_cell: params.target_cell,
                target_transform: params.target_transform,
                opened: false,
            })
            .insert(Animator::new(seq));
        commands.entity(parent).add_child(gate);

        //spawn hypergate output
        commands.spawn((
            SceneBundle {
                scene: assets.load("hypergate_out.glb#Scene0"),
                transform: Transform::from_translation(
                    params.target_transform.translation - Vec3::new(10.0, 0.0, 0.0),
                )
                .looking_at(params.target_transform.translation, Vec3::Y),
                ..default()
            },
            params.target_cell.clone(),
            Name::new("hypergate_output".to_string()),
            HypergateOutput {
                start_time: time.elapsed(),
            },
        ));
    }
}

fn small_hypergate_time_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SmallHypergate)>,
    time: Res<Time>,
) {
    for (entity, mut hypergate) in query.iter_mut() {
        let dt = time.elapsed() - hypergate.start_time;
        if dt >= Duration::from_secs_f32(GATE_BASE_TIME * 1.5) {
            commands.entity(entity).despawn_recursive();
        }
        if dt >= Duration::from_secs_f32(GATE_BASE_TIME * 0.5) {
            hypergate.opened = true;
        }
    }
}

fn small_hypergate_output(
    mut commands: Commands,
    mut query: Query<(Entity, &mut HypergateOutput)>,
    time: Res<Time>,
) {
    for (entity, output) in query.iter_mut() {
        let dt = time.elapsed() - output.start_time;
        if dt >= Duration::from_secs_f32(GATE_BASE_TIME * 3.0) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
