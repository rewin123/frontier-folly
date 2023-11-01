use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::{object::ship::Ship, position::{SpaceCell, SpaceCellPercision}};

pub struct FighterControllerPlugin;

impl Plugin for FighterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FighterControler>()
            .add_event::<FighterControlerEvent>()
            .add_systems(
                PostUpdate,
                (
                    default_input_map,
                    fighter_controller_system,
                    smoother_system,
                )
                    .chain().before(big_space::recenter_transform_on_grid::<SpaceCellPercision>),
            );
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct ParentSmoother {
    pub parent: Option<Entity>,
    pub target: Vec3,
    pub eye: Vec3,
    pub smoothing_weight: f32,

    pub current_eye: Option<Vec3>,
    pub current_target: Option<Vec3>,
}

impl Default for ParentSmoother {
    fn default() -> Self {
        Self {
            parent: None,
            target: Vec3::ZERO,
            eye: Vec3::ONE * 10.0,
            smoothing_weight: 0.98,
            current_eye: None,
            current_target: None,
        }
    }
}

fn smoother_system(
    mut smoothers: Query<(&mut ParentSmoother, &mut Transform)>,
    parents: Query<&Transform, Without<ParentSmoother>>,
    time: Res<Time>,
) {
    for (mut parent_smoother, mut transform) in smoothers.iter_mut() {
        let Some(parent) = parent_smoother.parent else {
            warn!("ParentSmoother has no parent");
            continue;
        };

        let Ok(parent_transform) = parents.get(parent) else {
            warn!("ParentSmoother has no parent transform");
            continue;
        };

        let cur_eye = parent_smoother.current_eye.unwrap_or(parent_smoother.eye)
            * parent_smoother.smoothing_weight
            + parent_smoother.eye * (1.0 - parent_smoother.smoothing_weight);
        let cur_target = parent_smoother
            .current_target
            .unwrap_or(parent_smoother.target)
            * parent_smoother.smoothing_weight
            + parent_smoother.target * (1.0 - parent_smoother.smoothing_weight);

        parent_smoother.current_eye = Some(cur_eye);
        parent_smoother.current_target = Some(cur_target);

        transform.translation = parent_transform.translation + cur_eye;
        let up: Vec3 = transform.up();
        transform.look_at(parent_transform.translation + cur_target, up);
    }
}

#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct FighterControllerConfig {
    pub max_speed: f32,
    pub ramp_up_time : f32
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct FighterControler {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub ship_rotate_sensitivity: f32,
    pub transform_sensitivity: f32,
    pub radius: Option<f32>,
    pub dp: Option<Vec3>,
}

impl Default for FighterControler {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(2.0),
            transform_sensitivity: 0.1,
            ship_rotate_sensitivity: 20.0,
            radius: None,
            dp: None,
        }
    }
}

#[derive(Event)]
enum FighterControlerEvent {
    Rotate(Vec2),
    TranslateEye(f32),
    Move(Vec3),
}

fn default_input_map(
    mut events: EventWriter<FighterControlerEvent>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    controllers: Query<&FighterControler>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let FighterControler {
        mouse_rotate_sensitivity,
        transform_sensitivity,
        ..
    } = *controller;

    let Ok(window) = q_windows.get_single() else {
        return;
    };
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let cursor_pos = window.cursor_position().unwrap_or(window_size / 2.0);

    let mut cursor_dp = Vec2::new(
        window_size.x / 2.0 - cursor_pos.x,
        cursor_pos.y - window_size.y / 2.0,
    ) / window_size
        * 2.0;
    cursor_dp.x = cursor_dp.x * window_size.x / window_size.y;
    cursor_dp.x = cursor_dp.x.min(1.0).max(-1.0);
    cursor_dp = cursor_dp * cursor_dp.length();

    events.send(FighterControlerEvent::Rotate(
        mouse_rotate_sensitivity * cursor_dp,
    ));

    for event in mouse_wheel.iter() {
        events.send(FighterControlerEvent::TranslateEye(
            transform_sensitivity * event.y,
        ));
    }
}

fn fighter_controller_system(
    mut events: EventReader<FighterControlerEvent>,
    mut cameras: Query<
        (
            &mut FighterControler,
            &mut ParentSmoother,
            &mut Transform,
            &GlobalTransform,
        ),
        Without<Ship>,
    >,
    mut ships: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
) {
    let Ok((mut controller, mut smoother, mut transform, global_transform)) =
        cameras.get_single_mut()
    else {
        warn!("Fighter controller cannot find camera");
        return;
    };

    let Ok(mut ship_transform) = ships.get_mut(smoother.parent.unwrap()) else {
        warn!("Fighter controller cannot find ship");
        return;
    };

    let up_diff = ship_transform.up() - transform.up();
    let up_diff = -up_diff.dot(transform.right());
    transform.rotate_local_axis(Vec3::Z, up_diff * time.delta_seconds() * 5.0);

    let mut radius = if let Some(r) = controller.radius {
        r
    } else {
        (smoother.eye - smoother.target).length()
    };

    let mut dp = if let Some(dp) = controller.dp {
        dp
    } else {
        (smoother.eye - smoother.target).normalize_or_zero()
    };

    for event in events.iter() {
        match event {
            FighterControlerEvent::Rotate(v) => {
                let move_dir = time.delta_seconds()
                    * radius
                    * (transform.right() * v.x + transform.up() * v.y);

                dp += move_dir;
                dp = dp.normalize_or_zero() * radius;
                smoother.eye = dp + smoother.target;
            }
            FighterControlerEvent::TranslateEye(v) => {
                let step = radius * v;
                radius -= step;
                dp = dp.normalize_or_zero() * radius;
            }
            FighterControlerEvent::Move(mv) => {}
        }
    }

    controller.dp = Some(dp);
    controller.radius = Some(radius);
    smoother.eye = dp + smoother.target;

    smoother.target = smoother.target
        + (ship_transform.up() * (0.5 + radius * 0.1) - smoother.target) * time.delta_seconds();

    //rotate ship to forward
    let dp = smoother.eye - smoother.target;
    let dp = -dp.normalize_or_zero();
    let ship_frw = ship_transform.forward();
    let new_frw =
        ship_frw + (dp - ship_frw) * time.delta_seconds() * controller.ship_rotate_sensitivity;
    let ship_pos = ship_transform.translation;
    let ship_up = ship_transform.up();
    ship_transform.look_at(ship_pos + new_frw, ship_up);
}
