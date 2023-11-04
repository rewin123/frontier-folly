use bevy::prelude::*;

use crate::position::SpaceCell;

pub struct LaserBeamBulletPlugin;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub struct LaserBeamBulletSet;

impl Plugin for LaserBeamBulletPlugin {
    fn build(&self, app: &mut App) {

        app.configure_set(Update, LaserBeamBulletSet);

        app.add_event::<SpawnLaserBeamBullet>()
            .add_systems(
                Update,
                (
                    laser_beam_flying,
                    spawn_laser_beam_bullet
                ).in_set(LaserBeamBulletSet)
            );
    }
}

#[derive(Event)]
pub struct SpawnLaserBeamBullet {
    pub position: Vec3,
    pub cell : SpaceCell,
    pub direction: Vec3,
    pub color: Color,
    pub speed: f32,
    pub lifetime: f32,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LaserBeamBullet {
    pub color: Color,
    pub speed: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub prev_global_trasform: Option<GlobalTransform>,
}

fn laser_beam_flying(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut LaserBeamBullet, &mut Transform, &GlobalTransform)>,
    mut gizmos : Gizmos
) {
    for (entity, mut laser_beam, mut transform, global_transform) in query.iter_mut() {
        laser_beam.lifetime -= time.delta_seconds();
        if laser_beam.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            let frw = transform.forward();
            transform.translation += frw * laser_beam.speed * time.delta_seconds();
            gizmos.line(
                global_transform.translation(),
                global_transform.translation() + frw * laser_beam.speed * time.delta_seconds(),
                laser_beam.color * (laser_beam.lifetime / laser_beam.max_lifetime)
            );

            if let Some(prev) = &laser_beam.prev_global_trasform {
                gizmos.line(
                    prev.translation(),
                    global_transform.translation(),
                    laser_beam.color * (laser_beam.lifetime / laser_beam.max_lifetime)
                );
            }

            laser_beam.prev_global_trasform = Some(global_transform.clone());
        }
    }
}

fn spawn_laser_beam_bullet(
    mut commands: Commands,
    mut events : EventReader<SpawnLaserBeamBullet>,
    time: Res<Time>,
) {
    for event in events.iter() {
        commands.spawn((
            LaserBeamBullet {
                color: event.color,
                speed: event.speed,
                lifetime: event.lifetime,
                max_lifetime: event.lifetime,
                prev_global_trasform: None,
            },
            event.cell.clone(),
            PointLightBundle {
                transform : Transform::from_translation(event.position).looking_at(event.position + event.direction, Vec3::Y),
                point_light: PointLight {
                    color: event.color,
                    intensity: 1.0,
                    ..default()
                },
                ..default()
            }
        ));
    }
}