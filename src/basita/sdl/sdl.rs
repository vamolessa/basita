use sdl2;
use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};

use super::{FontStorage, TextureStorage};

pub struct SdlContext {
	_sdl: sdl2::Sdl,

	pub canvas: Canvas<Window>,
	pub event_pump: sdl2::EventPump,
}

impl SdlContext {
	pub fn new(window_title: &str, window_width: u32, window_height: u32) -> Self {
		let sdl = sdl2::init().unwrap();
		let video_subsystem = sdl.video().unwrap();

		let window = video_subsystem
			.window(window_title, window_width, window_height)
			.position_centered()
			.build()
			.unwrap();

		let canvas = window.into_canvas().build().unwrap();

		let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

		if false {
			let _ttf = sdl2::ttf::init().unwrap();
			let _font = _ttf.load_font("", 32).unwrap();
		}

		SdlContext {
			event_pump: sdl.event_pump().unwrap(),
			_sdl: sdl,
			canvas: canvas,
		}
	}
}

pub struct SdlLoader {
	pub texture_creator: TextureCreator<WindowContext>,
	pub ttf_context: Sdl2TtfContext,
}

use core::assets::AssetLoadError;

impl SdlLoader {
	pub fn new(sdl_context: &SdlContext) -> Self {
		SdlLoader {
			texture_creator: sdl_context.canvas.texture_creator(),
			ttf_context: ttf::init().unwrap(),
		}
	}

	pub fn load_texture<'a>(
		&'a self,
		path: &str,
		storage: &mut SdlStorage<'a>,
	) -> Result<usize, AssetLoadError> {
		match self.texture_creator.load_texture(path) {
			Ok(texture) => Ok(storage.texture_storage.add(texture)),
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}

	pub fn load_font<'a>(
		&'a self,
		path: &str,
		_size: usize,
		storage: &mut SdlStorage<'a>,
	) -> Result<usize, AssetLoadError> {
		match self.texture_creator.load_texture(path) {
			Ok(texture) => Ok(storage.texture_storage.add(texture)),
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}

#[derive(Default)]
pub struct SdlStorage<'a> {
	pub texture_storage: TextureStorage<'a>,
	pub font_storage: FontStorage<'a, 'a>,
}
