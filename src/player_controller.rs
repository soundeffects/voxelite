use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use std::f32::consts::FRAC_PI_2;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSettings>()
            .add_systems(Startup, setup)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, handle_keyboard_input)
            .add_systems(Update, handle_mouse_input)
            .add_systems(Update, cursor_grab);
    }
}

#[derive(Default, Component)]
pub struct PlayerController {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Resource)]
pub struct PlayerSettings {
    pub view_distance: usize,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            view_distance: 1,
            movement_speed: 20.0,
            mouse_sensitivity: 0.1,
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        })
        .insert(PlayerController {
            yaw: 0.0,
            pitch: 0.0,
            ..Default::default()
        });
}

fn toggle_grab_cursor(window: &mut Window) {
    window.cursor.visible = !window.cursor.visible;
    window.cursor.grab_mode = if window.cursor.visible {
        CursorGrabMode::None
    } else {
        CursorGrabMode::Locked
    };
}

fn initial_grab_cursor(mut window_query: Query<&mut Window>) {
    let Ok(mut primary) = window_query.get_single_mut() else {
        return;
    };
    toggle_grab_cursor(&mut primary);
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut window_query: Query<&mut Window>) {
    let Ok(mut primary) = window_query.get_single_mut() else {
        return;
    };
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(&mut primary);
    }
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window>,
    settings: Res<PlayerSettings>,
    mut query: Query<&mut Transform, With<PlayerController>>,
) {
    let Ok(primary) = window_query.get_single() else {
        return;
    };
    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            if primary.cursor.grab_mode == CursorGrabMode::Locked {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= right,
                    KeyCode::D => velocity += right,
                    KeyCode::Space => velocity += Vec3::Y,
                    KeyCode::ShiftLeft => velocity -= Vec3::Y,
                    _ => (),
                }
            }
        }

        velocity = velocity.normalize_or_zero();

        transform.translation += velocity * time.delta_seconds() * settings.movement_speed
    }
}

fn handle_mouse_input(
    settings: Res<PlayerSettings>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
    window_query: Query<&Window>,
    mut query: Query<(&mut Transform, &mut PlayerController)>,
) {
    let Ok(primary) = window_query.get_single() else {
        return;
    };
    if primary.cursor.grab_mode == CursorGrabMode::Locked {
        for (mut transform, mut player_controller) in query.iter_mut() {
            if let Some(rotation) = mouse_motion.read().map(|m| m.delta).reduce(|a, e| a + e) {
                let mut new_pitch = player_controller.pitch
                    + rotation.y * time.delta_seconds() * settings.mouse_sensitivity;
                let new_yaw = player_controller.yaw
                    + rotation.x * time.delta_seconds() * settings.mouse_sensitivity;

                new_pitch = new_pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

                player_controller.pitch = new_pitch;
                player_controller.yaw = new_yaw;

                transform.rotation = Quat::from_axis_angle(-Vec3::Y, new_yaw)
                    * Quat::from_axis_angle(-Vec3::X, new_pitch)
            }
        }
    }
}
