use std::cmp::Ordering;
use sdl2::rect::Rect;

use sdl_context::SdlContext;
use components::{ComponentCollection, Sprite};

pub struct RendererSystem {}

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

impl<'a> RendererSystem {
	pub fn update(
		&self,
		sdl: &mut SdlContext,
		sprite_collection: &mut ComponentCollection<Sprite<'a>>,
	) {
		sprite_collection.all.sort_unstable();

		for sprite in &sprite_collection.all {
			let query = sprite.image.texture.query();
			let transform = sprite.transform;

			sdl.canvas
				.copy(
					&sprite.image.texture,
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
