use sdl2;
use sdl2::pixels::Color;

pub struct Application {
	pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
	pub event_pump: sdl2::EventPump,
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
		canvas: canvas,
		event_pump: sdl_context.event_pump().unwrap(),
	}
}
