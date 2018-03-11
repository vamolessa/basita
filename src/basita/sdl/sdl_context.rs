use std::cell::RefCell;

use sdl2;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::render::{Canvas};
use sdl2::video::{Window};

use super::Textures;

pub struct SdlContext<'a> {
	_sdl: sdl2::Sdl,

	pub canvas: RefCell<Canvas<Window>>,
	pub event_pump: RefCell<sdl2::EventPump>,

	// assets
	pub textures: Textures<'a>,
}

impl<'a> SdlContext<'a> {
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
			textures: Textures::new(texture_creator),
		}
	}
}
