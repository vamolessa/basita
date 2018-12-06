use basita::sdl2::keyboard::Keycode;
use basita::sdl2::pixels::Color;
use basita::sdl2::rect::Point;
use basita::specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use basita::core::components::Transform;
use basita::gui::Gui;
use basita::input::Input;
use basita::math::Vector2;
use basita::mixer::resources::SfxCommands;
use basita::physics::components::PhysicBody;

use crate::game::components::Player;
use crate::game::resources::{GameFonts, GameSfxs};

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
	type SystemData = (
		Read<'a, Input>,
		ReadStorage<'a, Player>,
		WriteStorage<'a, PhysicBody>,
		WriteStorage<'a, Transform>,
		Gui<'a>,
		Write<'a, SfxCommands>,
		Read<'a, GameFonts>,
		Read<'a, GameSfxs>,
	);

	fn run(
		&mut self,
		(
			input,
			players,
			mut physic_bodies,
			mut transforms,
			mut gui,
			mut sfx_commands,
			game_fonts,
			game_sfxs,
		): Self::SystemData,
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
				sfx_commands.add_play(game_sfxs.hit);
			}

			physic_body.acceleration.y = 100.0;
		}

		gui.font_handle = game_fonts.consola_32;
		gui.layer = 100;
		gui.color = Color::RGB(255, 100, 100);
		gui.label(Point::new(10, 10), "Coe, lek", Vector2::zero());
	}
}
