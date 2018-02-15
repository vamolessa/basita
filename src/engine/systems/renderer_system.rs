use std::cmp::Ordering;
use sdl2::rect::Rect;

use sdl_context::SdlContext;
use resources::ImageResources;
use components::{ComponentCollection, Sprite};

pub struct RendererSystem {}

impl<'a> PartialEq for Sprite {
	fn eq(&self, other: &Self) -> bool {
		return self.depth == other.depth;
	}
}

impl<'a> Eq for Sprite {}

impl<'a> PartialOrd for Sprite {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<'a> Ord for Sprite {
	fn cmp(&self, other: &Self) -> Ordering {
		other.depth.cmp(&self.depth)
	}
}

impl<'a> RendererSystem {
	pub fn update(
		&self,
		sdl: &mut SdlContext,
		image_resources: &ImageResources<'a>,
		sprite_collection: &mut ComponentCollection<Sprite>,
	) {
		sprite_collection.all.sort_unstable();

		for sprite in &sprite_collection.all {
			let texture = image_resources.get_texture(sprite.image);

			let query = texture.query();
			let transform = super::super::components::Transform {
				position: super::super::math::Vector2::new(0.0, 0.0),
			};

			sdl.canvas
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
