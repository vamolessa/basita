extern crate sdl2;

use std::path::Path;

use sdl2::rect::Rect;

mod application;
mod input;
mod image_resources;

use input::Input;
use image_resources::ImageResources;

pub fn main() {
	let mut app = application::init("platform maker", 400, 300);

	let mut input = Input::new();
	let mut image_resources = ImageResources::new(&mut app);

	let player_image = image_resources.load(Path::new("resources/sprites/player.png"));

	application::run(
		app,
		|event| input.handle_event(event),
		|app| {
			let query = player_image.texture.query();
			app.canvas
				.copy(
					&player_image.texture,
					None,
					Rect::from((0, 0, query.width, query.height)),
				)
				.unwrap();
			true
		},
	);
}
