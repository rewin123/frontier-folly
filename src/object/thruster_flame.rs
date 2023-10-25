use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{render_resource::{AsBindGroup, ShaderRef}, primitives::CubemapFrusta}, asset::ChangeWatcher, core_pipeline::bloom::BloomSettings, pbr::CubemapVisibleEntities,
};
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
        app.add_systems(Update, (
            thruster_flame_system,
            add_material,
            add_point_light
        ));
            
        app.add_plugins(MaterialPlugin::<FlameMaterial>::default());

        app.editor_registry::<ThrusterFlame>();
    }
}

fn thruster_flame_system(
        mut flames : Query<(&mut Transform, &ThrusterFlame, &mut PointLight)>
) {
    for (mut transform, flame, mut light) in flames.iter_mut() {
        transform.scale = Vec3::new(transform.scale.x, flame.length * flame.scale_ratio, transform.scale.z);
        light.intensity = flame.length / flame.max_length * 20.0;
    }
}

fn add_material(
    mut commands : Commands,
    mut flames : Query<Entity, (With<ThrusterFlame>, Without<Handle<FlameMaterial>>)>,
    mut materials : ResMut<Assets<FlameMaterial>>,
) {
    for flame in flames.iter_mut() {
        commands.entity(flame).insert(materials.add(FlameMaterial {
            color: Color::ORANGE_RED,
        }));
    }
}

fn add_point_light(
    mut commands : Commands,
    mut flames : Query<Entity, (With<ThrusterFlame>, Without<PointLight>)>,
) {
    for flame in flames.iter_mut() {
        commands.entity(flame).insert((
            PointLight {
                color : Color::ORANGE_RED,
                ..default()
            },
            CubemapFrusta::default(),
            CubemapVisibleEntities::default()
        ));
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for FlameMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/flame_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct FlameMaterial {
    #[uniform(0)]
    pub color: Color,
}