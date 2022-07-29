use std::f32::consts::{PI, FRAC_PI_2};

use bevy::{
	prelude::*,
	input::mouse::MouseMotion
};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<PlayerSettings>()
			.add_startup_system(setup)
			.add_startup_system(initial_grab_cursor)
			.add_system(handle_keyboard_input)
			.add_system(handle_mouse_input)
			.add_system(cursor_grab);
	}
}

#[derive(Default, Component)]
pub struct PlayerController {
	pub yaw: f32,
	pub pitch: f32,
}

pub struct PlayerSettings {
	pub view_distance: usize,
	pub movement_speed: f32,
	pub mouse_sensitivity: f32,
	pub field_of_view: f32
}

impl Default for PlayerSettings {
	fn default() -> Self {
		Self {
			view_distance: 6,
			movement_speed: 20.0,
			mouse_sensitivity: 0.5,
			field_of_view: 0.5
		}
	}
}

fn setup(mut commands: Commands, settings: Res<PlayerSettings>) {
	commands.spawn_bundle(PerspectiveCameraBundle {
		perspective_projection: PerspectiveProjection {
			fov: PI * settings.field_of_view,
			..Default::default()
		},
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
	window.set_cursor_lock_mode(!window.cursor_locked());
	window.set_cursor_visibility(!window.cursor_visible());
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
	toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
	let window = windows.get_primary_mut().unwrap();
	if keys.just_pressed(KeyCode::Escape) {
		toggle_grab_cursor(window);
	}
}

fn handle_keyboard_input(
	keys: Res<Input<KeyCode>>,
	time: Res<Time>,
	windows: Res<Windows>,
	settings: Res<PlayerSettings>,
	mut query: Query<&mut Transform, With<PlayerController>>
) {
	let window = windows.get_primary().unwrap();
	for mut transform in query.iter_mut() {
		let mut velocity = Vec3::ZERO;
		let local_z = transform.local_z();
		let forward = -Vec3::new(local_z.x, 0., local_z.z);
		let right = Vec3::new(local_z.z, 0., -local_z.x);

		for key in keys.get_pressed() {
			if window.cursor_locked() {
				match key {
					KeyCode::W => velocity += forward,
					KeyCode::S => velocity -= forward,
					KeyCode::A => velocity -= right,
					KeyCode::D => velocity += right,
					KeyCode::Space => velocity += Vec3::Y,
					KeyCode::LShift => velocity -= Vec3::Y,
					_ => ()
				}
			}
		}

		velocity = velocity.normalize_or_zero();

		transform.translation +=
			velocity * time.delta_seconds() * settings.movement_speed
	}
}

fn handle_mouse_input(
	settings: Res<PlayerSettings>,
	mut mouse_motion: EventReader<MouseMotion>,
	time: Res<Time>,
	windows: Res<Windows>,
	mut query: Query<(&mut Transform, &mut PlayerController)>
) {
	let window = windows.get_primary().unwrap();
	if window.cursor_locked() {
		for (mut transform, mut player_controller) in query.iter_mut() {
			if let Some(rotation) = mouse_motion.iter().map(|m| m.delta)
				.reduce(|a, e| a + e) {
				let mut new_pitch =
					player_controller.pitch + rotation.y * time.delta_seconds()
					* settings.mouse_sensitivity;
				let new_yaw =
					player_controller.yaw + rotation.x * time.delta_seconds() *
					settings.mouse_sensitivity;

				new_pitch = new_pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

				player_controller.pitch = new_pitch;
				player_controller.yaw = new_yaw;

				transform.rotation = Quat::from_axis_angle(-Vec3::Y, new_yaw)
					* Quat::from_axis_angle(-Vec3::X, new_pitch)
			}
		}
	}
}
