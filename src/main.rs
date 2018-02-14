extern crate sdl2;

use sdl2::pixels::Color;

mod application;
mod input;

pub fn main() {
	let app = application::init("platform maker", 400, 300);
	let mut i = input::Input {};

	application::run(
		app,
		|e| i.handle_event(e),
		|a| {
			a.canvas.set_draw_color(Color::RGB(0, 64, 255));
			a.canvas.clear();
			true
		},
	);
}
