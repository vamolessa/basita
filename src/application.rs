use sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;

use std::time::Duration;

pub struct Application {
	pub sdl_context: sdl2::Sdl,
	pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
	pub event_pump: sdl2::EventPump,

	pub frames_per_second: u32,
}

pub fn init(window_title: &str, window_width: u32, window_height: u32) -> Application {
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

	Application {
		event_pump: sdl_context.event_pump().unwrap(),
		sdl_context: sdl_context,
		canvas: canvas,
		frames_per_second: 60,
	}
}

pub fn run<FE, FF>(mut app: Application, mut event_callback: FE, mut frame_callback: FF)
where
	FE: FnMut(Event) -> bool,
	FF: FnMut(&mut Application) -> bool,
{
	let clear_color = Color::RGB(0, 0, 0);

	'main: loop {
		for event in app.event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'main,
				_ => if !event_callback(event) {
					break 'main;
				},
			}
		}

		app.canvas.set_draw_color(clear_color);
		app.canvas.clear();

		if !frame_callback(&mut app) {
			break;
		}

		app.canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / app.frames_per_second));
	}
}
