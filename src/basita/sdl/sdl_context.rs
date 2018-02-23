use std::cell::RefCell;

use sdl2;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct SdlContext {
	_sdl: sdl2::Sdl,

	pub canvas: RefCell<Canvas<Window>>,
	pub event_pump: RefCell<sdl2::EventPump>,

	// resources
	pub texture_creator: TextureCreator<WindowContext>,
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
		let texture_creator = canvas.texture_creator();

		SdlContext {
			event_pump: RefCell::from(sdl.event_pump().unwrap()),
			_sdl: sdl,
			canvas: RefCell::from(canvas),
			texture_creator: texture_creator,
		}
	}
}
