use bevy::{prelude::*, input::common_conditions::input_toggle_active};
use bevy_egui::{EguiPlugin, EguiContexts, egui};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use big_space::FloatingOrigin;
use frontier_folly::{controller::{ControllerPlugin, OrbitControler, FighterControler, ParentSmoother}, object::{small_hypergate::SmallHypergatePlugin, ship::Ship}, position::SpaceCell, enviroment::sand_cloud::{SandCloudSpawner, SandCloudPlugin}};

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
        .add_systems(Update, (
            apply_velocity,
            ship_controller,
            enviroment_camera_follow
        ))
        .run();
}

fn setup(
    mut commands : Commands,
    assets : Res<AssetServer>,
    mut meshs : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>
) {

    let pipe_test = commands.spawn(SceneBundle {
        scene: assets.load("pipe_test.glb#Scene0"),
        transform: Transform::from_xyz(100.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
        ..default()
    });

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
        },
        Velocity::default()
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
        FighterControler::default(),
        ParentSmoother {
            parent : Some(ship),
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

    //enviroment sphere
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::UVSphere::default().into()),
            material : materials.add(StandardMaterial {
                base_color_texture : Some(assets.load("hdr.png")),
                unlit : true,
                double_sided : true,
                alpha_mode : AlphaMode::Add,
                // emissive : Color::WHITE,
                ..default()
            }),
            transform : Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(-1.0e+10)),
            ..default()
        },
        SpaceCell::default(),
        EnviromentSphere,
        Name::new("Enviroment Sphere")
    ));
}

fn ship_controller(
    mut ships : Query<(&mut Velocity, &Transform, &Ship)>,
    keys : Res<Input<KeyCode>>,
    time : Res<Time>,
    mut ctxs : EguiContexts
) {
    let acceleration = 1.0;
    let restriction = 0.5;
    let dt = time.delta_seconds();
    egui::Window::new("Ship Controller").show(ctxs.ctx_mut(), |ui| {
        ships.for_each_mut(|(mut velocity, transform, ship)| {
            let right = transform.right();
            
            if keys.pressed(KeyCode::W) {
                velocity.0 += transform.forward() * acceleration * dt;
            }
            if keys.pressed(KeyCode::S) {
                velocity.0 -= transform.forward() * acceleration * dt;
            }
    
            if keys.pressed(KeyCode::A) {
    
            }
    
            let vel = velocity.0;
            velocity.0 -= vel * restriction * dt;

            ui.label(format!("{:.1} km/h", velocity.0.length() * 3600.0 / 1000.0));
        })
    });
}

#[derive(Component, Default)]
struct Velocity(pub Vec3);

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation += velocity.0;
    });
}

#[derive(Component)]
pub struct EnviromentSphere;

fn enviroment_camera_follow(
    cameras : Query<&GlobalTransform, (With<Camera>, Without<EnviromentSphere>)>,
    mut env : Query<(&mut Transform, &GlobalTransform), With<EnviromentSphere>>
) {
    let cam_pos = cameras.single().translation();
    for (mut transform, global_transform) in env.iter_mut() {
        let dp = cam_pos - global_transform.translation();
        transform.translation += dp;
    }
}