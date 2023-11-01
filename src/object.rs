use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod ship;
pub mod small_hypergate;
pub mod thruster;
pub mod thruster_flame;
pub mod laser_gun;
pub mod laser_beam_bullet;

pub struct ObjectPlugins;

impl PluginGroup for ObjectPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(small_hypergate::SmallHypergatePlugin)
            .add(thruster_flame::ThrusterFlamePlugin)
            .add(laser_beam_bullet::LaserBeamBulletPlugin)
            .add(laser_gun::LaserGunPlugin)
    }
}
