use sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;

use std::time::Duration;

pub struct SdlContext {
	pub sdl_context: sdl2::Sdl,
	pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
	pub event_pump: sdl2::EventPump,

	pub frames_per_second: u32,
}

impl SdlContext {
	pub fn new(window_title: &str, window_width: u32, window_height: u32) -> SdlContext {
		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();

		let window = video_subsystem
			.window(window_title, window_width, window_height)
			.position_centered()
			.build()
			.unwrap();

		let mut canvas = window.into_canvas().build().unwrap();
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		canvas.present();

		SdlContext {
			event_pump: sdl_context.event_pump().unwrap(),
			sdl_context: sdl_context,
			canvas: canvas,
			frames_per_second: 60,
		}
	}

	pub fn run<FE, FF>(&mut self, mut event_callback: FE, mut frame_callback: FF)
	where
		FE: FnMut(Event) -> bool,
		FF: FnMut() -> bool,
	{
		let clear_color = Color::RGB(0, 0, 0);

		'main: loop {
			for event in self.event_pump.poll_iter() {
				match event {
					Event::Quit { .. } => break 'main,
					_ => if !event_callback(event) {
						break 'main;
					},
				}
			}

			self.canvas.set_draw_color(clear_color);
			self.canvas.clear();

			if !frame_callback() {
				break;
			}

			self.canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.frames_per_second));
		}
	}
}
