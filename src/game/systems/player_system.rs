use basita::sdl2::keyboard::Keycode;
use basita::specs::{Join, Read, ReadStorage, System, WriteStorage};

use basita::core::components::Transform;
use basita::input::Input;
use basita::physics::components::PhysicBody;

use components::Player;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
	type SystemData = (
		Read<'a, Input>,
		ReadStorage<'a, Player>,
		WriteStorage<'a, PhysicBody>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (input, players, mut physic_bodies, mut transforms): Self::SystemData) {
		let move_velocity = 60.0;
		let jump_impulse = 100.0;

		for (_player, physic_body, _transform) in
			(&players, &mut physic_bodies, &mut transforms).join()
		{
			physic_body.velocity.x = 0.0;
			if input.key(Keycode::Left).is_pressed {
				physic_body.velocity.x -= move_velocity;
			}
			if input.key(Keycode::Right).is_pressed {
				physic_body.velocity.x += move_velocity;
			}

			if input.key(Keycode::Up).just_pressed() {
				physic_body.velocity.y -= jump_impulse;
			}

			physic_body.acceleration.y = 100.0;
		}
	}
}
