use bevy::prelude::*;
use space_editor::prelude::EditorRegistryExt;

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct ThrusterFlame {
    pub length: f32,
    pub scale_ratio : f32,
    pub max_length: f32,
}

impl Default for ThrusterFlame {
    fn default() -> Self {
        Self {
            length: 0.0,
            scale_ratio: 1.0,
            max_length: 1.0,
        }
    }
}

pub struct ThrusterFlamePlugin;

impl Plugin for ThrusterFlamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, thruster_flame_system);

        app.editor_registry::<ThrusterFlame>();
    }
}

fn thruster_flame_system(
        mut flames : Query<(&mut Transform, &ThrusterFlame)>
) {
    for (mut transform, flame) in flames.iter_mut() {
        transform.scale = Vec3::new(1.0, flame.length * flame.scale_ratio, 1.0);
    }
}
