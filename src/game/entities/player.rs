use basita::{
	core::{components::Transform, resources::LazyEvaluations},
	math::Vector2,
	physics::components::{Collider, PhysicBody},
	renderer::components::Sprite,
};

use crate::MyGame;

pub struct PlayerEntity {
	pub transform: Transform,
	pub sprite: Sprite,
	pub collider: Collider,
	pub physic_body: PhysicBody,
}

pub fn new(lazy: &mut LazyEvaluations<MyGame>, position: Vector2) {
	/*
	lazy.add(move |context, game| {
		let image_handle;
		{
			image_handle = game.images.load(
				&String::from("assets/images/player.png"),
				context.sdl_loader,
				&mut context.sdl_storage,
			);
		}

		game.players.push(PlayerEntity {
			transform: Transform { position: position },
			sprite: Sprite {
				image: image_handle,
				..Default::default()
			},
			collider: Collider::new_box(Vector2::new(16.0, 16.0)),
			physic_body: PhysicBody::with_inverted_mass(1.0),
		});
	});
	*/
}
