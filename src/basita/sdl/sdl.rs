use sdl2;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::{FontLoader, FontStorage, TextureLoader, TextureStorage};

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

		SdlContext {
			event_pump: sdl.event_pump().unwrap(),
			_sdl: sdl,
			canvas: canvas,
		}
	}
}

pub struct SdlLoader {
	pub texture_loader: TextureLoader,
	pub font_loader: FontLoader,
}

impl SdlLoader {
	pub fn new(sdl_context: &SdlContext) -> Self {
		SdlLoader {
			texture_loader: TextureLoader::new(sdl_context),
			font_loader: FontLoader::new(),
		}
	}
}

#[derive(Default)]
pub struct SdlStorage<'a> {
	pub texture_storage: TextureStorage<'a>,
	pub font_storage: FontStorage<'a, 'a>,
}
