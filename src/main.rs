extern crate sdl2;

use std::path::Path;

mod application;
mod input;
mod math;

mod components;
mod systems;
mod resources;

use input::Input;
use math::Vector2;
use components::{BoxCollider, ComponentCollection, Sprite, Transform};
use resources::ImageResources;

pub fn main() {
	let mut app = application::init("platform maker", 400, 300);

	let mut input = Input::new();
	let mut image_resources = ImageResources::new(&mut app);

	let player_image = image_resources.load(Path::new("resources/sprites/player.png"));

	let mut transform_collection = ComponentCollection::new();
	let mut sprite_collection = ComponentCollection::new();
	let mut box_collider_collection = ComponentCollection::new();

	let transform = transform_collection.add(Transform::default());

	sprite_collection.add(Sprite {
		transform: transform,
		depth: 0,
		image: &player_image,
	});

	box_collider_collection.add(BoxCollider {
		transform: transform,
		size: Vector2::from((32.0, 32.0)),
		offset: Vector2::default(),
	});

	let mut renderer_system = systems::renderer_system::RendererSystem {};

	application::run(
		app,
		|event| input.handle_event(event),
		|app| {
			renderer_system.update(app, &mut sprite_collection, &mut box_collider_collection);
			true
		},
	);
}
