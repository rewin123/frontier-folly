
use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::position::{SpacePosition, SpaceCell};

pub struct DebugControllerPlugin;

impl Plugin for DebugControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DebugController>()
            .add_event::<DebugControlEvent>()
            .add_systems(Update, (default_input_map, debug_controller_system).chain());
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct DebugController {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub translate_sensitivity: f32,
    pub smoothing_weight: f32,
}

impl Default for DebugController {
    fn default() -> Self {
        Self {  
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(0.2),
            translate_sensitivity: 10000.0,
            smoothing_weight: 0.9,
        }
    }
}

#[derive(Event)]
enum DebugControlEvent {
    Rotate(Vec2),
    TranslateEye(Vec3),
}

fn default_input_map(
    mut events: EventWriter<DebugControlEvent>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    controllers: Query<&DebugController>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let DebugController {
        translate_sensitivity,
        mouse_rotate_sensitivity,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    events.send(DebugControlEvent::Rotate(
        mouse_rotate_sensitivity * cursor_delta,
    ));

    for (key, dir) in [
        (KeyCode::W, Vec3::Z),
        (KeyCode::A, Vec3::X),
        (KeyCode::S, -Vec3::Z),
        (KeyCode::D, -Vec3::X),
        (KeyCode::ShiftLeft, -Vec3::Y),
        (KeyCode::Space, Vec3::Y),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            events.send(DebugControlEvent::TranslateEye(translate_sensitivity * dir));
        }
    }
}

fn debug_controller_system(
    mut events: EventReader<DebugControlEvent>,
    mut cameras: Query<(&DebugController, &mut Transform)>,
    time: Res<Time>,
) {

    let (controller, mut transform) = if let Some((controller, transform)) = cameras.iter_mut().find(|c| c.0.enabled) {
        (controller, transform)
    } else {
        return;
    };


    for event in events.iter() {
        match event {
            DebugControlEvent::Rotate(v) => {
                let target = transform.translation + transform.forward() + (-transform.up() * v.y * controller.mouse_rotate_sensitivity.x + transform.right() * v.x * controller.mouse_rotate_sensitivity.y) * time.delta_seconds();
                transform.look_at(target, Vec3::Y);
            },
            DebugControlEvent::TranslateEye(v) => {
                let move_dir = (transform.forward() * v.z + transform.up() * v.y + transform.right() * v.x).normalize_or_zero();
                transform.translation += move_dir * time.delta_seconds() * controller.translate_sensitivity;
            },
        }
    }
}