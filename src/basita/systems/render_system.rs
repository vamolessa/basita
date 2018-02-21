use std::cmp::Ordering;

use sdl2::rect::Rect;

use super::super::{ContainsEngineEvents, ContainsEngineState};
use super::System;

use components::Sprite;

pub struct RenderSystem;

impl<'a, S, E> System<S,E> for RenderSystem
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	fn update(s: &mut S, _e: &E) {
		let state = s.get_engine_state_mut();

		state.sprites.all.sort_unstable();

		let mut canvas = state.sdl_context.canvas.borrow_mut();

		for sprite in &state.sprites.all {
			let texture = &state.image_resources.get(sprite.image_resource).texture;
			let query = texture.query();

			let transform = state.transforms.get(sprite.transform);

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
