use basita::sdl2::keyboard::Keycode;
use basita::specs::{Join, Read, ReadStorage, System, WriteStorage};

use basita::core::components::Transform;
use basita::input::Input;

use components::Player;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
	type SystemData = (
		Read<'a, Input>,
		ReadStorage<'a, Player>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (input, players, mut transforms): Self::SystemData) {
		for (_player, transform) in (&players, &mut transforms).join() {
			if input.key(Keycode::Left).is_pressed {
				transform.position.x -= 1.0;
			}
			if input.key(Keycode::Right).is_pressed {
				transform.position.x += 1.0;
			}
			if input.key(Keycode::Up).is_pressed {
				transform.position.y -= 1.0;
			}
			if input.key(Keycode::Down).is_pressed {
				transform.position.y += 1.0;
			}
		}
	}
}
