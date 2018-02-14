extern crate sdl2;

use std::path::Path;

mod application;
mod input;
mod components;
mod systems;

mod image_resources;

use input::Input;

use components::sprite::Sprite;
use image_resources::ImageResources;

pub fn main() {
	let mut app = application::init("platform maker", 400, 300);

	let mut input = Input::new();
	let mut image_resources = ImageResources::new(&mut app);

	let player_image = image_resources.load(Path::new("resources/sprites/player.png"));

	let mut sprites = components::sprite::Sprites::new();
	sprites.add(Sprite {
		x: 0,
		y: 10,
		image: &player_image,
	});

	let renderer_system = systems::renderer_system::RendererSystem { sprites: &sprites };

	application::run(
		app,
		|event| input.handle_event(event),
		|app| {
			renderer_system.update(app);
			true
		},
	);
}
