use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use big_space::FloatingOrigin;
use frontier_folly::{
    controller::{ControllerPlugin, FighterControler, ParentSmoother},
    enviroment::sand_cloud::{SandCloudPlugin, SandCloudSpawner},
    object::{ship::Ship, small_hypergate::SmallHypergatePlugin, ObjectPlugins},
    position::SpaceCell,
};
use space_editor::prelude::{PrefabPlugin, PrefabBundle, load::PrefabLoader};

const CURSOR_SIZE: f32 = 40.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugins(EguiPlugin)
        .add_plugins((big_space::FloatingOriginPlugin::<i64>::default(),))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)),
        )
        .add_plugins(PrefabPlugin)
        .add_plugins(bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin)
        .add_plugins(ControllerPlugin)
        .add_plugins(ObjectPlugins)
        .add_plugins(SandCloudPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                apply_velocity,
                ship_controller,
                enviroment_camera_follow,
                cursor_pos_system,
                update_cursor_visiblity,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct CursorNode;

fn cursor_pos_system(
    mut cursors: Query<(&mut Style), With<CursorNode>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        for mut style in cursors.iter_mut() {
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(position.y - CURSOR_SIZE / 2.0);
            style.left = Val::Px(position.x - CURSOR_SIZE / 2.0);
        }
    } else {
    }
}

fn update_cursor_visiblity(
    mut q_windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    mut ctxs: EguiContexts,
) {
    let ctx = ctxs.ctx_mut();
    let mut visible = false;
    if ctx.is_pointer_over_area() || ctx.is_using_pointer() {
        visible = true;
    } else {
        visible = false;
    }
    for mut window in q_windows.iter_mut() {
        window.cursor.visible = visible;
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cursor_texture = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(CURSOR_SIZE),
                height: Val::Px(CURSOR_SIZE),
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        UiImage::new(assets.load("cursor_2.png")),
        CursorNode,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(2.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(
                    109.0 / 255.0,
                    188.0 / 255.0,
                    185.0 / 255.0,
                    0.5,
                )),
                ..default()
            });
        });

    let pipe_test = commands.spawn(SceneBundle {
        scene: assets.load("pipe_test.glb#Scene0"),
        transform: Transform::from_xyz(100.0, 0.0, 0.0).with_scale(Vec3::splat(4.0)),
        ..default()
    });

    let ship = commands
        .spawn((
            PrefabBundle::new("test_ship.scn.ron"),
            SpaceCell::default(),
            Name::new("Ship"),
            Ship,
            SandCloudSpawner { ..default() },
            Velocity::default(),
        ))
        .id();

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
                ..default()
            },
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        bevy::core_pipeline::bloom::BloomSettings { ..default() },
        SpaceCell::default(), // All spatial entities need this component
        FloatingOrigin,       // Important: marks this as the entity to use as the floating origin
        FighterControler::default(),
        ParentSmoother {
            parent: Some(ship),
            ..default()
        },
        bevy::pbr::ScreenSpaceAmbientOcclusionBundle::default(),
    ));

    // light
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config: bevy::pbr::CascadeShadowConfigBuilder { ..default() }.into(),
        ..default()
    },));

    //enviroment sphere
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::UVSphere::default().into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("hdr.png")),
                unlit: true,
                double_sided: true,
                alpha_mode: AlphaMode::Add,
                // emissive : Color::WHITE,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(-1.0e+10)),
            ..default()
        },
        SpaceCell::default(),
        EnviromentSphere,
        Name::new("Enviroment Sphere"),
    ));
}

fn ship_controller(
    mut ships: Query<(&mut Velocity, &Transform, &Ship)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ctxs: EguiContexts,
) {
    let acceleration = 1.0;
    let restriction = 0.9;
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

            if keys.pressed(KeyCode::A) {}

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
    cameras: Query<&GlobalTransform, (With<Camera>, Without<EnviromentSphere>)>,
    mut env: Query<(&mut Transform, &GlobalTransform), With<EnviromentSphere>>,
) {
    let cam_pos = cameras.single().translation();
    for (mut transform, global_transform) in env.iter_mut() {
        let dp = cam_pos - global_transform.translation();
        transform.translation += dp;
    }
}
