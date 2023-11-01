use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef}, ecs::{entity::MapEntities, reflect::ReflectMapEntities}
};
use space_editor::{prelude::{EditorRegistryExt, component::EntityLink}, EditorSet};

#[derive(Component, Reflect, Clone)]
#[reflect(Component, MapEntities)]
pub struct ThrusterFlame {
    pub length: f32,
    pub scale_ratio : f32,
    pub max_length: f32,
    pub look_vector : Vec3,
    pub point_light : EntityLink
}

impl Default for ThrusterFlame {
    fn default() -> Self {
        Self {
            length: 0.0,
            scale_ratio: 1.0,
            max_length: 1.0,
            point_light: EntityLink::default(),
            look_vector: Vec3::Z
        }
    }
}

impl MapEntities for ThrusterFlame {
    fn map_entities(&mut self, entity_mapper: &mut bevy::ecs::entity::EntityMapper) {
        self.point_light.entity = entity_mapper.get_or_reserve(self.point_light.entity);
    }
}

pub struct ThrusterFlamePlugin;

impl Plugin for ThrusterFlamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            thruster_flame_system,
            add_material
        ));
            
        app.add_plugins(MaterialPlugin::<FlameMaterial>::default());

        app.editor_registry::<ThrusterFlame>();

        app.add_systems(Update, editor_debug_draw.in_set(EditorSet::Editor));
    }
}

fn editor_debug_draw(
    mut flames : Query<(&Transform, &ThrusterFlame)>,
    mut gizmos : Gizmos
) {
    for (transform, flame) in flames.iter() {
        gizmos.line(
            transform.translation,
            transform.translation + flame.length * (flame.look_vector.x * transform.right() + flame.look_vector.y * transform.up() + flame.look_vector.z * transform.forward()),
            Color::ORANGE_RED
        );
    }
}

fn thruster_flame_system(
        mut flames : Query<(&mut Transform, &ThrusterFlame)>,
        mut lights : Query<&mut PointLight>
) {
    for (mut transform, flame) in flames.iter_mut() {
        transform.scale = Vec3::new(transform.scale.x, flame.length * flame.scale_ratio, transform.scale.z);
        if flame.point_light.entity != Entity::PLACEHOLDER {
            if let Ok(mut light) = lights.get_mut(flame.point_light.entity) {
                light.intensity = flame.length / flame.max_length * 20.0;
            } else {
                warn!("ThrusterFlame point light not found {:?}", flame.point_light.entity);
            }
        }
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