use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}};

pub struct OrbitControllerPlugin;

impl Plugin for OrbitControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OrbitControler>()
            .add_event::<OrbitControlerEvent>()
            .add_systems(Update, (default_input_map, debug_controller_system).chain());
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct OrbitControler {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub smoothing_weight: f32,
    pub transform_sensitivity: f32,
    pub target : Option<Entity>,
    pub radius : Option<f32>,
}

impl Default for OrbitControler {
    fn default() -> Self {
        Self { 
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(0.2),
            transform_sensitivity: 0.1,
            smoothing_weight: 0.9,
            target: None,
            radius: None,
        }
    }
}

#[derive(Event)]
enum OrbitControlerEvent {
    Rotate(Vec2),
    TranslateEye(f32),
}


fn default_input_map(
    mut events: EventWriter<OrbitControlerEvent>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel : EventReader<MouseWheel>,
    controllers: Query<&OrbitControler>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let OrbitControler {
        mouse_rotate_sensitivity,
        transform_sensitivity,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    events.send(OrbitControlerEvent::Rotate(
        mouse_rotate_sensitivity * Vec2::new(-cursor_delta.x, cursor_delta.y),
    ));

    for event in mouse_wheel.iter() {
        events.send(OrbitControlerEvent::TranslateEye(transform_sensitivity * event.y));
    }
}

fn debug_controller_system(
    mut events: EventReader<OrbitControlerEvent>,
    mut cameras: Query<(&mut OrbitControler, &mut Transform, &GlobalTransform)>,
    mut targets : Query<&GlobalTransform, Without<OrbitControler>>,
    time: Res<Time>,
) {

    let (mut controller, mut transform, global_transform) = if let Some((controller, transform, global_transform)) = cameras.iter_mut().find(|c| c.0.enabled) {
        (controller, transform, global_transform)
    } else {
        return;
    };

    let Some(target_entity) = controller.target else {
        return;
    };

    let Ok(target) = targets.get(target_entity) else {
        warn!("Could not find target");
        return;
    };

    {   //Fix distance to target before moving (for case when target moving)
        let dp =  target.translation() - global_transform.translation();
        let radius = if let Some(radius) = controller.radius {
            radius
        } else {
            controller.radius = Some(dp.length());
            info!("Set orbit controller radius to {}", controller.radius.unwrap());
            controller.radius.unwrap()
        };
        let virtual_target = transform.translation + dp;
        let new_dp = dp.normalize_or_zero() * radius;
        transform.translation += (dp - new_dp);
    }


    for event in events.iter() {
        match event {
            OrbitControlerEvent::Rotate(v) => {
                let dp =  target.translation() - global_transform.translation();
                let radius = dp.length();
                let virtual_target = transform.translation + dp;

                let move_dir = time.delta_seconds() * radius * (transform.right() * v.x + transform.up() * v.y);   
                let up = transform.up();
                transform.translation += move_dir;
                transform.look_at(virtual_target, up);

                let new_r = (transform.translation - virtual_target).length();
                let step = new_r - radius;
                let frw = transform.forward();
                transform.translation += frw * step;
            },
            OrbitControlerEvent::TranslateEye(v) => {
                let dp = target.translation() - global_transform.translation();
                let radius = dp.length();
                
                let step = radius * v;
                controller.radius = Some(radius - step);
                let frw = transform.forward();
                transform.translation += frw * step;
            },
        }
    }
}