use sdl2::keyboard::Keycode;

use engine::input::{Button, Input};
use engine::components::Transform;

pub struct PlayerSystem {
	left_button: Button,
	right_button: Button,
	jump_button: Button,
}

impl PlayerSystem {
	pub fn new(input: &mut Input) -> PlayerSystem {
		PlayerSystem {
			left_button: input.new_button(Keycode::Left),
			right_button: input.new_button(Keycode::Right),
			jump_button: input.new_button(Keycode::Space),
		}
	}

	pub fn update(&self, input: &Input, player_transform: &mut Transform) {
		if input.is_pressed(self.left_button) {
			player_transform.position.x -= 1.0;
		}
		if input.is_pressed(self.right_button) {
			player_transform.position.x += 1.0;
		}

		if input.was_pressed(self.jump_button) {
			player_transform.position.y -= 10.0;
		}

		if input.was_released(self.jump_button) {
			player_transform.position.y += 10.0;
		}
	}
}
