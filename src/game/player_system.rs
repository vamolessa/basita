use sdl2::keyboard::Keycode;

use basita::EngineState;
use basita::input::Button;
use basita::math::Vector2;
use basita::components::*;

use game::*;

pub struct PlayerSystemData {
	pub left_button: Button,
	pub right_button: Button,
	pub jump_button: Button,

	pub player_transform_handle: ComponentHandle<Transform>,
}

impl PlayerSystemData {
	pub fn new(state: &mut EngineState) -> Self {
		PlayerSystemData {
			left_button: state.input.new_button(Keycode::Left),
			right_button: state.input.new_button(Keycode::Right),
			jump_button: state.input.new_button(Keycode::Space),

			player_transform_handle: state.transforms.add(Transform::identity()),
		}
	}
}

pub struct PlayerSystem {}

impl GameSystem for PlayerSystem {
	fn init(state: &mut GameState, _events: &mut GameEvents) {
		let player_image = state
			.engine_state
			.image_resources
			.load(&String::from("resources/sprites/player.png"));

		let transform = state
			.engine_state
			.transforms
			.get_mut(state.player_system_data.player_transform_handle);
		transform.position = Vector2::new(10.0, 200.0);

		state.engine_state.sprites.add(Sprite {
			depth: 0,
			image_resource: player_image,
			transform: state.player_system_data.player_transform_handle,
		});

		state.engine_state.colliders.add(Collider {
			shape: Shape::Box(BoxShape {
				half_size: Vector2::from((16.0, 16.0)),
			}),
			offset: Vector2::zero(),
			enabled: true,
			is_trigger: false,

			transform: state.player_system_data.player_transform_handle,
			physic_body: None,
		});
	}

	fn update(state: &mut GameState, _events: &GameEvents) {
		let player_transform = state
			.engine_state
			.transforms
			.get_mut(state.player_system_data.player_transform_handle);

		if state
			.engine_state
			.input
			.is_pressed(state.player_system_data.left_button)
		{
			player_transform.position.x -= 1.0;
		}
		if state
			.engine_state
			.input
			.is_pressed(state.player_system_data.right_button)
		{
			player_transform.position.x += 1.0;
		}

		if state
			.engine_state
			.input
			.was_pressed(state.player_system_data.jump_button)
		{
			player_transform.position.y -= 10.0;
		}

		if state
			.engine_state
			.input
			.was_released(state.player_system_data.jump_button)
		{
			player_transform.position.y += 10.0;
		}
	}
}
