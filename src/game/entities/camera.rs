use basita::{core::resources::LazyEvaluations, math::Vector2, renderer::components::Camera};

use crate::MyGame;

pub struct CameraEntity {
	pub camera: Camera,
}

pub fn new(lazy: &mut LazyEvaluations<MyGame>, position: Vector2) {
	lazy.add(move |_context, game| {
		game.cameras.push(CameraEntity {
			camera: Camera { position: position },
		});
	});
}
