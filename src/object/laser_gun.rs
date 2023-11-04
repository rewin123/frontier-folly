use bevy::prelude::*;

use crate::position::SpaceCell;

use super::laser_beam_bullet::SpawnLaserBeamBullet;


pub struct LaserGunPlugin;

impl Plugin for LaserGunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_from_laser_gun);
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct LaserGun {
    pub need_fire: bool,

    pub beam_color : Color,
    pub beam_brightness : f32,
    pub beam_speed : f32,
    pub beam_lifetime : f32,
}

fn fire_from_laser_gun(
    mut laser_guns: Query<(&Transform, &SpaceCell, &mut LaserGun)>,
    mut spawn_events: EventWriter<SpawnLaserBeamBullet>,
) {
    for (transform, cell, mut laser_gun) in laser_guns.iter_mut() {
        if laser_gun.need_fire {
            spawn_events.send(SpawnLaserBeamBullet {
                position: transform.translation,
                cell : cell.clone(),
                direction: transform.forward(),
                color: laser_gun.beam_color,
                speed: laser_gun.beam_speed,
                lifetime: laser_gun.beam_lifetime,
            });
            laser_gun.need_fire = false;
        }
    }
}

