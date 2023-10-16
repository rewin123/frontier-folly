use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

pub struct SandCloudPlugin;

impl Plugin for SandCloudPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SandCloudSpawner>()
            .register_type::<SandGrain>()
            .add_systems(Startup, setup_global)
            .add_systems(Update, sand_cloud_update);
    }
}


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SandGrain;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SandCloudSpawner {
    pub density: f32,
    pub radius: f32,
    pub check_distance_patience: f32,
}

#[derive(Resource)]
pub struct SandCloudGlobal {
    pub grain_mesh: Handle<Mesh>,
    pub grain_material: Handle<StandardMaterial>,
}

impl Default for SandCloudSpawner {
    fn default() -> Self {
        Self { 
            density: 0.0001,
            radius: 160.0,
            check_distance_patience: 1.0,
        }
    }
}

fn setup_global(
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
) {
    let grain_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.1 }));
    let grain_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        emissive: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });
    commands.insert_resource(SandCloudGlobal {
        grain_mesh,
        grain_material,
    });
}

fn sand_cloud_update(
    mut commands : Commands,
    global : Res<SandCloudGlobal>,
    grains : Query<(Entity, &GlobalTransform, &SandGrain)>,
    mut spawners : Query<(Entity, &GlobalTransform, &SandCloudSpawner)>,
) {
    let (_, spawner_transform, spawner) = spawners.single_mut();

    let mut grain_count = 0;
    //destroy far grains
    for (entity, transform, grain) in &grains {
        if (transform.translation() - spawner_transform.translation()).length() > spawner.radius {
            commands.entity(entity).despawn();
        } else {
            grain_count += 1;
        }
    }

    let spawner_volume = 3.0 / 4.0 * PI * spawner.radius.powf(3.0);
    let need_count = (spawner.density * spawner_volume) as i32;

    if grain_count < need_count {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..=1.0);
        for _ in 0..(need_count - grain_count) {          
            commands.spawn((
                PbrBundle {
                    mesh: global.grain_mesh.clone(),
                    material: global.grain_material.clone(),
                    transform: Transform::from_xyz(
                        rng.gen_range(-spawner.radius..=spawner.radius) + spawner_transform.translation().x,
                        rng.gen_range(-spawner.radius..=spawner.radius) + spawner_transform.translation().y,
                        rng.gen_range(-spawner.radius..=spawner.radius) + spawner_transform.translation().z,
                    ),
                    ..default()
                },
                SandGrain,
            ));
        }
    }
}