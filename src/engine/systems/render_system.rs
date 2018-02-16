use std::cmp::Ordering;

use sdl2::rect::Rect;

use super::super::EngineState;
use super::System;

use components::Sprite;

impl<'a> PartialEq for Sprite<'a> {
	fn eq(&self, other: &Self) -> bool {
		return self.depth == other.depth;
	}
}

impl<'a> Eq for Sprite<'a> {}

impl<'a> PartialOrd for Sprite<'a> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<'a> Ord for Sprite<'a> {
	fn cmp(&self, other: &Self) -> Ordering {
		other.depth.cmp(&self.depth)
	}
}

pub struct RenderSystem {}

impl System for RenderSystem {
	fn update(&mut self, engine: &mut EngineState) {
		engine.sprites.all.sort_unstable();

		let mut canvas = engine.sdl_context.canvas.borrow_mut();

		for sprite in &engine.sprites.all {
			let texture = &engine.image_resources.get(sprite.image_resource).texture;
			let query = texture.query();

			let transform = super::super::components::Transform {
				position: super::super::math::Vector2::new(0.0, 0.0),
			};

			canvas
				.copy(
					texture,
					None,
					Rect::new(
						transform.position.x as i32,
						transform.position.y as i32,
						query.width,
						query.height,
					),
				)
				.unwrap();
		}
	}
}
