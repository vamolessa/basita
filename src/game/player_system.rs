use sdl2::keyboard::Keycode;

use basita::EngineState;
use basita::systems::System;
use basita::input::Button;
use basita::math::Vector2;
use basita::components::*;

pub struct PlayerSystem {
	left_button: Button,
	right_button: Button,
	jump_button: Button,

	player_transform_handle: ComponentHandle<Transform>,
}

impl PlayerSystem {
	pub fn new(state: &mut EngineState) -> Self {
		PlayerSystem {
			left_button: state.input.new_button(Keycode::Left),
			right_button: state.input.new_button(Keycode::Right),
			jump_button: state.input.new_button(Keycode::Space),

			player_transform_handle: state.transforms.add(Transform::identity()),
		}
	}
}

impl System for PlayerSystem {
	fn init(&self, state: &mut EngineState) {
		let player_image = state
			.image_resources
			.load(&String::from("resources/sprites/player.png"));

		let transform = state.transforms.get_mut(self.player_transform_handle);
		transform.position = Vector2::new(10.0, 200.0);

		state.sprites.add(Sprite {
			depth: 0,
			image_resource: player_image,
			transform: self.player_transform_handle,
		});

		state.box_colliders.add(BoxCollider {
			size: Vector2::from((32.0, 32.0)),
			offset: Vector2::zero(),
			transform: self.player_transform_handle,
		});
	}

	fn update(&self, state: &mut EngineState) {
		let player_transform = state.transforms.get_mut(self.player_transform_handle);

		if state.input.is_pressed(self.left_button) {
			player_transform.position.x -= 1.0;
		}
		if state.input.is_pressed(self.right_button) {
			player_transform.position.x += 1.0;
		}

		if state.input.was_pressed(self.jump_button) {
			player_transform.position.y -= 10.0;
		}

		if state.input.was_released(self.jump_button) {
			player_transform.position.y += 10.0;
		}
	}
}
