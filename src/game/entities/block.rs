use basita::sdl2::pixels::Color;

use basita::{
	core::{components::Transform, resources::LazyEvaluations},
	math::Vector2,
	physics::components::Collider,
	renderer::components::Sprite,
};

use crate::MyGame;

pub struct BlockEntity {
	pub transform: Transform,
	pub sprite: Sprite,
	pub collider: Collider,
}

pub fn new(lazy: &mut LazyEvaluations<MyGame>, position: Vector2) {
	/*
	lazy.add(move |context, game| {
		let image_handle;
		{
			image_handle = game.images.load(
				&String::from("assets/images/block.png"),
				context.sdl_loader,
				&mut context.sdl_storage,
			);
		}

		game.blocks.push(BlockEntity {
			transform: Transform { position: position },
			sprite: Sprite {
				color: Color::RGB(128, 128, 200),
				image: image_handle,
				..Default::default()
			},
			collider: Collider::new_box(Vector2::new(16.0, 16.0)),
		});
	});
	*/
}
