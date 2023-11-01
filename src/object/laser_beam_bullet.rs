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
    pub length: f32,
    pub color: Color,
    pub speed: f32,
    pub lifetime: f32,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LaserBeamBullet {
    pub length: f32,
    pub color: Color,
    pub speed: f32,
    pub lifetime: f32,
    pub max_lifetime: f32
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
                global_transform.translation() + frw * laser_beam.length,
                laser_beam.color * (laser_beam.lifetime / laser_beam.max_lifetime)
            );
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
            SpatialBundle::from_transform(Transform::from_translation(event.position).looking_at(event.position + event.direction, Vec3::Y)),
            LaserBeamBullet {
                length: event.length,
                color: event.color,
                speed: event.speed,
                lifetime: event.lifetime,
                max_lifetime: event.lifetime
            },
            event.cell.clone()
        ));
    }
}