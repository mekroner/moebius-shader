use bevy::{
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::post_process::PostProcessSettings;

const CAMERA_HEIGHT: f32 = 10.;
pub struct FlyCamPlugin;

impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, cursor_grab)
            .add_systems(Update, cam_look)
            .add_systems(Update, cam_move);
    }
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

#[derive(Resource)]
struct MovementSettings {
    sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            toggle_grab_cursor: KeyCode::Tab,
        }
    }
}

#[derive(Component)]
pub struct FlyCam;

fn spawn_camera(mut cmds: Commands) {
    let translation = Vec3::new(0.0, CAMERA_HEIGHT, 10.0);

    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam,
        PostProcessSettings { intensity: 0.01 },
        NormalPrepass,
        DepthPrepass,
    ));
}

fn toggle_grab_cursor(window: &mut Window) {
    if window.cursor.grab_mode == CursorGrabMode::None {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
        return;
    }
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut window) = primary_window.get_single_mut() else {
        warn!("Primary window not found for `cursor_grab`!");
        return;
    };
    if keys.just_pressed(key_bindings.toggle_grab_cursor) {
        toggle_grab_cursor(&mut window);
    }
}

fn cam_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: ResMut<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let Ok(window) = primary_window.get_single() else {
        warn!("Primary window not found!");
        return;
    };

    if window.cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    for mut transform in query.iter_mut() {
        for ev in state.reader_motion.read(&motion) {
            let window_scale = window.height().min(window.width());
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
            yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
            pitch = pitch.clamp(-1.54, 1.54);
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn cam_move(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let Ok(window) = primary_window.get_single() else {
        warn!("Primary window not found!");
        return;
    };

    if window.cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    for mut transform in query.iter_mut() {
        let mut vel = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for &key in keys.get_pressed() {
            if key == key_bindings.move_forward {
                vel += forward;
            }
            if key == key_bindings.move_backward {
                vel -= forward;
            }
            if key == key_bindings.move_right {
                vel += right;
            }
            if key == key_bindings.move_left {
                vel -= right;
            }
            if key == key_bindings.move_ascend {
                vel += Vec3::Y;
            }
            if key == key_bindings.move_descend {
                vel -= Vec3::Y;
            }
        }

        vel = vel.normalize_or_zero();
        transform.translation += vel * time.delta_seconds() * settings.speed;
    }
}
