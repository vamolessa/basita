use std::time::Instant;

use basita::sdl2::event::Event;
use basita::{
	core::resources::{LazyEvaluations, Time},
	game::*,
	input::Input,
	mixer::resources::Sfxs,
	mixer::Mixer,
	renderer::{
		resources::{Fonts, Images},
		Renderer,
	},
};

mod game;
use crate::game::{
	entities::{block::*, camera::*, player::*},
	levels::*,
	resources::{GameFonts, GameSfxs},
};

#[derive(Default)]
pub struct MyGame {
	pub time: Time,
	pub input: Input,
	pub renderer: Renderer,
	pub mixer: Mixer,
	pub lazy_evaluations: LazyEvaluations<Self>,

	pub images: Images,
	pub fonts: Fonts,
	pub sfxs: Sfxs,

	pub game_fonts: GameFonts,
	pub game_sfxs: GameSfxs,

	pub cameras: Vec<CameraEntity>,
	pub players: Vec<PlayerEntity>,
	pub blocks: Vec<BlockEntity>,
}

impl Game for MyGame {
	fn create(_context: &mut GameContext) -> Self {
		let game = MyGame::default();

		//level1::load(lazy: &mut LazyEvaluations<MyGame>);

		game
	}

	fn run(&mut self, context: &mut GameContext) {
		'main: loop {
			let frame_start_instant = Instant::now();

			let event_pump = &mut context.sdl_context.event_pump;
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. } => {
						break 'main;
					}
					e => {
						self.input.handle_event(e);
					}
				};
			}

			// update

			self.renderer
				.render(&mut context.sdl_context, &mut context.sdl_storage)
				.unwrap();
			self.mixer.mix(&mut context.sdl_storage).unwrap();
			self.input.update();

			self.time
				.sleep_rest_of_frame(context.settings.frames_per_second, &frame_start_instant);
		}
	}
}

pub fn main() {
	MyGame::play();
}
