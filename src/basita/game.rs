use std::time::Instant;

use crate::sdl2::event::Event;

use crate::{
	core::resources::Time,
	input::Input,
	mixer::Mixer,
	//renderer::systems::RenderSystem,
	//physics::systems::{ColliderRenderSystem, PhysicsSystem},
	renderer::Renderer,
	sdl::{SdlContext, SdlLoader, SdlStorage},
};

pub struct GameSettings {
	pub title: String,
	pub frames_per_second: u32,
	pub screen_width: u16,
	pub screen_height: u16,
	pub simultaneous_audio_count: u8,
}

impl Default for GameSettings {
	fn default() -> Self {
		GameSettings {
			title: "game".into(),
			frames_per_second: 30,
			screen_width: 400,
			screen_height: 300,
			simultaneous_audio_count: 8,
		}
	}
}

#[derive(Default)]
pub struct GameResources {
	pub time: Time,
	pub input: Input,
	pub renderer: Renderer,
	pub mixer: Mixer,
}

pub struct GameContext<'a> {
	settings: GameSettings,

	sdl_context: SdlContext,
	sdl_loader: &'a mut SdlLoader,
	sdl_storage: SdlStorage<'a>,
}

pub trait Game: Sized {
	fn create(_context: &mut GameContext) -> Self;
	fn settings() -> GameSettings {
		GameSettings::default()
	}

	fn update(&mut self, _resources: &mut GameResources) {}

	fn run(&mut self, context: &mut GameContext) {
		let mut resources = GameResources::default();

		'main: loop {
			let frame_start_instant = Instant::now();

			let event_pump = &mut context.sdl_context.event_pump;
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. } => {
						break 'main;
					}
					e => {
						resources.input.handle_event(e);
					}
				};
			}

			self.update(&mut resources);
			resources
				.renderer
				.render(&mut context.sdl_context, &mut context.sdl_storage);
			resources.mixer.mix(&mut context.sdl_storage);
			resources.input.update();

			resources
				.time
				.sleep_rest_of_frame(context.settings.frames_per_second, &frame_start_instant);
		}
	}

	fn play() {
		let settings = Self::settings();

		let sdl_context = SdlContext::new(
			&settings.title[..],
			settings.screen_width as u32,
			settings.screen_height as u32,
			settings.simultaneous_audio_count,
		);
		let mut sdl_loader = SdlLoader::new(&sdl_context);
		let sdl_storage = SdlStorage::default();

		let mut context = GameContext {
			settings: settings,
			sdl_context: sdl_context,
			sdl_loader: &mut sdl_loader,
			sdl_storage: sdl_storage,
		};

		let mut game = Self::create(&mut context);
		game.run(&mut context);
	}
}
