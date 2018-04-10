use sdl2::rect::{Point, Rect};

use specs::{Fetch, FetchMut, Join, ReadStorage, System};

use super::assets::Image;
use super::components::Sprite;
use super::resources::{DirtySprites, ImageCollection};
use core::assets::AssetHandle;
use core::components::Transform;
use sdl::{SdlContext, SdlStorage};

struct Renderable {
	pub depth: i32,
	pub image: AssetHandle<Image>,
	pub position: Point,
}

impl Default for Renderable {
	fn default() -> Self {
		Renderable {
			depth: 0,
			image: Default::default(),
			position: Point::new(0, 0),
		}
	}
}

pub struct RenderSystem<'a: 'b, 'b> {
	sdl_context: &'a SdlContext,
	sdl_storage: &'b SdlStorage<'a>,
	renderables: Vec<Renderable>,
}

impl<'a, 'b> RenderSystem<'a, 'b> {
	pub fn new(sdl_context: &'a SdlContext, sdl_storage: &'b SdlStorage<'a>) -> Self {
		RenderSystem {
			sdl_context: sdl_context,
			sdl_storage: sdl_storage,
			renderables: Vec::default(),
		}
	}
}

impl<'a, 'b, 's> System<'s> for RenderSystem<'a, 'b> {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Sprite>,
		FetchMut<'s, DirtySprites>,
		Fetch<'s, ImageCollection>,
	);

	fn run(
		&mut self,
		(transforms, sprites, mut dirty_sprites, image_collection): Self::SystemData,
	) {
		if dirty_sprites.entities.len() > 0 {
			for entity in &dirty_sprites.entities {
				if let Some(sprite) = sprites.get(*entity) {
					if sprite.renderable_index >= self.renderables.len() {
						self.renderables.resize_default(sprite.renderable_index + 1);
					}

					let renderable = &mut self.renderables[sprite.renderable_index];
					renderable.depth = sprite.depth;
					renderable.image = sprite.image;
				}
			}

			self.renderables.sort_by(|a, b| a.depth.cmp(&b.depth));
			dirty_sprites.entities.clear();
		}

		for (transform, sprite) in (&transforms, &sprites).join() {
			let renderable = &mut self.renderables[sprite.renderable_index];
			renderable.position.x = transform.position.x as i32;
			renderable.position.y = transform.position.y as i32;
		}

		let mut canvas = self.sdl_context.canvas.borrow_mut();
		let textures = self.sdl_storage.texture_storage.borrow();

		for r in &self.renderables {
			let image = image_collection.get(r.image);
			let texture = textures.at(image.texture_index);

			let position = r.position + image.center;
			let rect = Rect::from_center(position, image.width, image.height);

			canvas.copy(texture, None, rect).unwrap();
		}
	}
}
