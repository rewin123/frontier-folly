use bevy::{
    core_pipeline::bloom::BloomSettings, ecs::system::EntityCommands,
    input::keyboard::KeyboardInput, prelude::*,
};
use bevy_egui::egui::Key;

const GATE_BUILDER_COUNT: usize = 6;
const GATE_RADIUS: f32 = 4.0;
const GATE_BUILDER_SPEED: f32 = 2.0;
const GATE_SPAWN_DIST: f32 = 20.0;
const GATE_CHARGE_TIME: f32 = 2.0;
const BEAM_COLOR: Color = Color::rgb(0.0, 10.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build())
        .add_systems(Startup, (camera_setup, setup_ship))
        .add_systems(
            Update,
            (
                start_move.before(spawn_hypergate),
                start_smooth_move,
                linear_move,
                spawn_hypergate,
                smooth_move,
                charge_hypergate,
                portal_edges,
                portal_edges_in_process,
                spawn_hypergate_mesh,
            ),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

#[derive(Component)]
struct Idle;

#[derive(Component)]
struct Hypergate {
    builders: Vec<Entity>,
    spawned: bool,
}

#[derive(Component)]
struct HypergateBuilderFinishedLinear;

#[derive(Component, Default)]
struct HypergateBuilderCharging {
    amount: f32,
}

#[derive(Component)]
struct ChargedHypergateBuilder;

#[derive(Component)]
struct HypergateBuilder {
    target: Transform,
    neighbors: Vec<Entity>,
}

#[derive(Component)]
struct LinearMoveTo {
    speed: f32,
    target: Transform,
    on_finish: Box<dyn Fn(&mut EntityCommands) + Send + Sync>,
}

#[derive(Component)]
struct SmoothMoveTo {
    speed: f32,
    target: Transform,
    on_finish: Box<dyn Fn(&mut EntityCommands) + Send + Sync>,
}

fn spawn_hypergate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hypergates: Query<(&mut Hypergate)>,
    builders: Query<(&Transform, &ChargedHypergateBuilder)>,
) {
    for (mut hypergate) in hypergates.iter_mut() {
        if !hypergate.spawned {
            let mut charged = 0;
            let mut mean_pos = Vec3::ZERO;
            for i in 0..GATE_BUILDER_COUNT {
                if let Ok((builder, _)) = builders.get(hypergate.builders[i]) {
                    charged += 1;
                    mean_pos += builder.translation
                }
            }
            if charged == GATE_BUILDER_COUNT {
                mean_pos /= GATE_BUILDER_COUNT as f32;

                hypergate.spawned = true;

                //spawn hypergate
                commands.spawn(PbrBundle {
                    mesh: meshes.add(
                        shape::RegularPolygon::new(GATE_RADIUS * 0.99, GATE_BUILDER_COUNT).into(),
                    ),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BLACK,
                        emissive: Color::BLUE,
                        ..default()
                    }),
                    transform: Transform::from_translation(mean_pos),
                    ..default()
                });
            }
        }
    }
}

fn portal_edges(
    mut gizmos: Gizmos,
    mut query: Query<(&Transform, &HypergateBuilder, &ChargedHypergateBuilder)>,
    mut charged: Query<(&Transform, &ChargedHypergateBuilder)>,
) {
    for (transform, hypergate, _) in query.iter_mut() {
        for neighbor in hypergate.neighbors.iter() {
            if let Ok((neighbor_transform, _)) = charged.get(*neighbor) {
                gizmos.line(
                    transform.translation,
                    neighbor_transform.translation,
                    BEAM_COLOR,
                );
            }
        }
    }
}

fn portal_edges_in_process(
    mut gizmos: Gizmos,
    mut query: Query<(&Transform, &HypergateBuilder, &HypergateBuilderCharging)>,
    mut charged: Query<(&Transform, &HypergateBuilderCharging)>,
    mut finished: Query<(&Transform, &ChargedHypergateBuilder)>,
) {
    for (transform, hypergate, charging) in query.iter_mut() {
        let k = charging.amount / GATE_CHARGE_TIME;
        for neighbor in hypergate.neighbors.iter() {
            if let Ok((neighbor_transform, neighbook_charge)) = charged.get(*neighbor) {
                let n_k = neighbook_charge.amount / GATE_CHARGE_TIME;
                gizmos.line(
                    transform.translation,
                    neighbor_transform.translation,
                    BEAM_COLOR * ((k + n_k) / 2.0),
                );
            }
            if let Ok((neighbor_transform, _)) = finished.get(*neighbor) {
                let n_k = 1.0;
                gizmos.line(
                    transform.translation,
                    neighbor_transform.translation,
                    BEAM_COLOR * ((k + n_k) / 2.0),
                );
            }
        }
    }
}

fn charge_hypergate(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut HypergateBuilderCharging)>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    let charge_pos = Vec3::new(0.0, 0.0, -2.0);
    let dt = time.delta_seconds().max(1.0 / 30.0);

    let mut counter = 0;
    for (entity, transform, mut charging) in query.iter_mut() {
        charging.amount += dt;
        if charging.amount >= GATE_CHARGE_TIME {
            commands
                .entity(entity)
                .remove::<HypergateBuilderCharging>()
                .insert(ChargedHypergateBuilder);
        }

        gizmos.line(charge_pos, transform.translation, BEAM_COLOR);
        counter += 1;
        if counter > 0 {
            break;
        }
    }
}

fn linear_move(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &LinearMoveTo)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds().max(1.0 / 30.0);

    for (entity, mut transform, linear) in query.iter_mut() {
        let dp = linear.target.translation - transform.translation;
        if dp.length() < 0.1 {
            commands.entity(entity).remove::<LinearMoveTo>();
            (linear.on_finish)(&mut commands.entity(entity));
        } else {
            transform.translation +=
                (dp.normalize_or_zero() * linear.speed * dt).clamp_length_max(dp.length());
        }
    }
}

fn smooth_move(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &SmoothMoveTo)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds().max(1.0 / 30.0);

    for (entity, mut transform, smooth) in query.iter_mut() {
        let dp = smooth.target.translation - transform.translation;
        let dq = smooth.target.rotation.xyz() - transform.rotation.xyz();
        if dp.length() < 0.1 && dq.length() < 0.1 {
            commands.entity(entity).remove::<SmoothMoveTo>();
            (smooth.on_finish)(&mut commands.entity(entity));
        } else {
            transform.translation +=
                (dp.normalize_or_zero() * smooth.speed * dt).clamp_length_max(dp.length());
            transform.rotation.x += dq.x * smooth.speed * dt;
            transform.rotation.y += dq.y * smooth.speed * dt;
            transform.rotation.z += dq.z * smooth.speed * dt;
        }
    }
}

fn start_smooth_move(
    mut commands: Commands,
    mut query: Query<(Entity, &HypergateBuilder), (With<HypergateBuilderFinishedLinear>)>,
) {
    for (entity, builder) in query.iter() {
        commands
            .entity(entity)
            .remove::<HypergateBuilderFinishedLinear>()
            .insert(SmoothMoveTo {
                speed: GATE_BUILDER_SPEED,
                target: builder.target,
                on_finish: Box::new(|entity| {
                    entity.insert(HypergateBuilderCharging::default());
                }),
            });
    }
}

fn start_move(
    mut commands: Commands,
    mut query: Query<Entity, (With<HypergateBuilder>, With<Idle>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .remove::<Idle>()
            .insert(LinearMoveTo {
                speed: GATE_BUILDER_SPEED,
                target: Transform::from_translation(Vec3::new(0.0, 0.0, -GATE_SPAWN_DIST / 3.0)),
                on_finish: Box::new(|entity| {
                    entity.insert(HypergateBuilderFinishedLinear);
                }),
            });
    }
}

fn spawn_hypergate(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if input.just_released(KeyCode::Space) {
        let mesh = meshes.add(shape::Box::new(0.1, 0.1, 0.1).into());
        let mat = materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        });

        let gate_position = Vec3::new(0.0, 0.0, -10.0);

        let mut entities = vec![];
        for i in 0..GATE_BUILDER_COUNT {
            entities.push(commands.spawn_empty().id());
        }

        for i in 0..GATE_BUILDER_COUNT {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / GATE_BUILDER_COUNT as f32;
            let target = Vec3::new(
                gate_position.x + GATE_RADIUS * angle.sin(),
                gate_position.y + GATE_RADIUS * angle.cos(),
                gate_position.z,
            );
            commands.entity(entities[i]).insert((
                PbrBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, i as f32 * 0.11))
                        .looking_at(gate_position, Vec3::Z),
                    mesh: mesh.clone(),
                    material: mat.clone(),
                    ..default()
                },
                Idle,
                HypergateBuilder {
                    target: Transform::from_translation(target)
                        .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                    neighbors: vec![
                        entities[(i + 1) % GATE_BUILDER_COUNT],
                        entities[(i + GATE_BUILDER_COUNT - 1) % GATE_BUILDER_COUNT],
                    ],
                },
            ));
        }

        commands.spawn(Hypergate {
            builders: entities,
            spawned: false,
        });
    }
}

fn setup_ship(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: assets.load("low_poly_fighter.glb#Scene0"),
        ..default()
    });
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
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
