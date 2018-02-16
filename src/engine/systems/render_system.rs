use std::cmp::Ordering;

use sdl2::rect::Rect;

use SdlContext;
use resources::ImageResources;
use components::{ComponentCollection, Sprite};

pub struct RenderSystem {}

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

impl<'a> RenderSystem {
	pub fn update(
		&self,
		sdl: &SdlContext,
		image_resources: &ImageResources<'a>,
		sprite_collection: &mut ComponentCollection<Sprite>,
	) {
		sprite_collection.all.sort_unstable();

		let mut canvas = sdl.canvas.borrow_mut();

		for sprite in &sprite_collection.all {
			let texture = &image_resources.get(sprite.image_resource).texture;
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
