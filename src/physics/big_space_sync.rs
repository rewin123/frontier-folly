use bevy::{prelude::*, math::DVec3};
use bevy_xpbd_3d::{prelude::*, SubstepSet, SubstepSchedule};

use crate::position::SpaceCellPercision;

use super::PhysicsOrigin;

pub struct SyncPlugin;

impl Plugin for SyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                fix_origin_to_zero,
                sync_transforms
            ).chain()
            .in_set(PhysicsSet::Sync),
        );
    }
}


type RbSyncQueryComponents = (
    &'static mut Transform,
    &'static Position,
    &'static Rotation,
    Option<&'static Parent>,
    &'static GlobalTransform
);

type RbSyncQueryFilter = Or<(Changed<Position>, Changed<Rotation>)>;

type RigidBodyParentComponents = (
    &'static GlobalTransform,
    Option<&'static Position>,
    Option<&'static Rotation>,
);

fn fix_origin_to_zero(
    mut bodies : Query<&mut Position, Without<PhysicsOrigin>>,
    mut origins : Query<(&mut Transform, &mut Position), With<PhysicsOrigin>>
) {
    let (mut origin_transform, mut origin_pos) = origins.single_mut();

    for mut body_pos in &mut bodies {
        body_pos.0 -= origin_pos.0;
    }

    origin_transform.translation += origin_pos.0.as_vec3();

    origin_pos.0 = DVec3::ZERO;
}

fn sync_transforms(
    mut bodies: Query<RbSyncQueryComponents, RbSyncQueryFilter>,
    parents: Query<RigidBodyParentComponents, With<Children>>,
    origins : Query<(&GlobalTransform), With<PhysicsOrigin>>
) {
    let (origin) = origins.single();

    for (mut transform, pos, rot, parent, global_transform) in &mut bodies {
        if let Some(parent) = parent {
            if let Ok((parent_transform, parent_pos, parent_rot)) = parents.get(**parent) {
                // Compute the global transform of the parent using its Position and Rotation
                // let parent_transform = parent_transform.compute_transform();
                // let parent_pos =
                //     parent_pos.map_or(parent_transform.translation, |pos| pos.as_vec3());
                // let parent_rot = parent_rot.map_or(parent_transform.rotation, |rot| rot.as_f32());
                // let parent_scale = parent_transform.scale;
                // let parent_transform = Transform::from_translation(parent_pos)
                //     .with_rotation(parent_rot)
                //     .with_scale(parent_scale);

                // // The new local transform of the child body,
                // // computed from the its global transform and its parents global transform
                // let new_transform = GlobalTransform::from(
                //     Transform::from_translation(pos.as_vec3()).with_rotation(rot.as_f32()),
                // )
                // .reparented_to(&GlobalTransform::from(parent_transform));

                // transform.translation = new_transform.translation;
                // transform.rotation = new_transform.rotation;
            }
        } else {
            let physics_pos = global_transform.translation() - origin.translation();
            transform.translation += pos.as_vec3() - physics_pos;
            transform.rotation = rot.as_f32();
        }
    }
}