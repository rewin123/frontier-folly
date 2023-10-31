use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

//Try to draw lasers in bevy

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build())
        .add_systems(Startup, setup)
        .add_systems(Update, draw_lasers)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0)
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
}

fn draw_lasers(mut gizmo: Gizmos) {
    let start = Vec3::new(-5.0, 0.0, 0.0);
    let end = Vec3::new(5.0, 0.0, 0.0);
    gizmo.line(start, end, Color::rgb(0.0, 30.0, 0.0));
}
