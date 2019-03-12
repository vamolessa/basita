use crate::sdl::{SdlContext, SdlLoader, SdlStorage};

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

pub struct GameContext<'a, 'b> {
	pub settings: GameSettings,

	pub sdl_context: &'a mut SdlContext,
	pub sdl_loader: &'a SdlLoader,
	pub sdl_storage: &'b mut SdlStorage<'a>,
}

pub trait Game: Sized {
	fn create(_context: &mut GameContext) -> Self;
	fn settings() -> GameSettings {
		GameSettings::default()
	}

	fn run(&mut self, context: &mut GameContext);

	fn play() {
		let settings = Self::settings();

		let mut sdl_context = SdlContext::new(
			&settings.title[..],
			settings.screen_width as u32,
			settings.screen_height as u32,
			settings.simultaneous_audio_count,
		)
		.unwrap();
		let sdl_loader = SdlLoader::new(&sdl_context).unwrap();
		let mut sdl_storage = SdlStorage::default();

		let mut context = GameContext {
			settings: settings,
			sdl_context: &mut sdl_context,
			sdl_loader: &sdl_loader,
			sdl_storage: &mut sdl_storage,
		};

		let mut game = Self::create(&mut context);
		game.run(&mut context);
	}
}
