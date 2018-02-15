extern crate sdl2;

use std::path::Path;

mod engine;
mod game;

use engine::*;

use application::Application;
use input::Input;
use math::Vector2;
use components::{BoxCollider, ComponentCollection, Sprite, Transform};
use resources::ImageResources;

use game::*;
use player_system::PlayerSystem;

pub fn main() {
	let mut application = Application::new("platform maker", 400, 300);

	let mut input = Input::new();
	let mut image_resources = ImageResources::new(&mut application);

	let player_image = image_resources.load(Path::new("resources/sprites/player.png"));

	let mut transform_collection = ComponentCollection::new();
	let mut sprite_collection = ComponentCollection::new();
	let mut box_collider_collection = ComponentCollection::new();

	transform_collection.add(Transform::default());

	let last_transform_index = transform_collection.all.len() - 1;
	let transform = &mut transform_collection.all[last_transform_index];
	transform.position = Vector2::new(10.0, 10.0);

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

	let renderer_system = systems::RendererSystem {};
	let collider_renderer_system = systems::ColliderRendererSystem {};

	let mut player_system = PlayerSystem::new(&mut input);

	application.run(
		|event| input.handle_event(event),
		|app| {
			player_system.update(&input, transform);
			renderer_system.update(app, &mut sprite_collection);
			collider_renderer_system.update(app, &box_collider_collection);
			true
		},
	);
}
