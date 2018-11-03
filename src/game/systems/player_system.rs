use basita::sdl2::keyboard::Keycode;
use basita::sdl2::pixels::Color;
use basita::sdl2::rect::Point;
use basita::specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use basita::core::components::Transform;
use basita::gui::Gui;
use basita::input::Input;
use basita::math::Vector2;
use basita::physics::components::PhysicBody;
use basita::renderer::resources::{Fonts, RenderCommands};

use components::Player;
use resources::GameFonts;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
	type SystemData = (
		Read<'a, Input>,
		ReadStorage<'a, Player>,
		WriteStorage<'a, PhysicBody>,
		WriteStorage<'a, Transform>,
		Read<'a, Fonts>,
		Write<'a, RenderCommands>,
		Read<'a, GameFonts>,
	);

	fn run(
		&mut self,
		(input, players, mut physic_bodies, mut transforms, fonts, mut render_commands, game_fonts): Self::SystemData,
	) {
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

		let consola_32_font = fonts.get(game_fonts.consola_32);
		let mut gui = Gui::new(&mut render_commands, &consola_32_font);
		gui.layer = 100;
		gui.color = Color::RGB(255, 100, 100);
		gui.label(Point::new(10, 10), "Coe, lek", Vector2::zero());
	}
}
