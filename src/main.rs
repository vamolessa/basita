extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::pixels::Color;

mod application;

pub fn main() {
	let mut app = application::init("platform maker", 400, 300);

	let mut i = 0;
	'main: loop {
		i = (i + 1) % 255;
		app.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
		app.canvas.clear();

		//app.canvas.set_draw_color(Color::RGB(255, 255, 255));
		//app.canvas.draw_line((0, 0), (100, 150)).unwrap();

		for event in app.event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'main,
				_ => {}
			}
		}
		// The rest of the game loop goes here...

		app.canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
