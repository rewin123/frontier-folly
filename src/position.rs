
use bevy::prelude::*;
use big_space::*;

/// Grid type for all game
pub type SpaceCell = GridCell<i64>;

#[derive(Reflect, Default, Clone, Copy, Debug, PartialEq)]
pub struct SpacePosition {
    pub cell : SpaceCell,
    pub position : Vec3,
}

impl SpacePosition {
    pub fn smooth(&self, new_position: &SpacePosition, lag_weight : f32, grid_settings: &big_space::FloatingOriginSettings) -> SpacePosition {
        //old_lerp_tfm.eye * self.lag_weight + new_tfm.eye * lead_weight,
        // new_tfm.eye + (old_lerp_tfm.eye - new_tfm.eye) * self.lag_weight

        let sub = *self - *new_position;
        let sub_dvec = grid_settings.grid_position_double(&sub.cell, &Transform::from_translation(sub.position));
        let sub_dvec = sub_dvec * lag_weight as f64;
        let sub = grid_settings.translation_to_grid(sub_dvec);
        let sub = SpacePosition { cell: sub.0, position: sub.1 };
        *new_position + sub
    }
}

impl std::ops::Add<SpacePosition> for SpacePosition {
    type Output = SpacePosition;

    fn add(self, rhs: SpacePosition) -> Self::Output {
        Self {
            cell: self.cell + rhs.cell,
            position: self.position + rhs.position,
        }
    }
}

impl std::ops::Sub<SpacePosition> for SpacePosition {
    type Output = SpacePosition;

    fn sub(self, rhs: SpacePosition) -> Self::Output {
        Self {
            cell: self.cell - rhs.cell,
            position: self.position - rhs.position,
        }
    }
}