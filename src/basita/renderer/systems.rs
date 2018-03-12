use sdl2::rect::Rect;
use sdl2::render::Texture;

use specs::{Join, ReadStorage, System, Fetch, FetchMut};

use sdl::SdlContext;
use core::components::Transform;
use core::assets::AssetHandle;
use super::components::Sprite;
use super::assets::Image;
use super::resources::{UpdatedSprites, ImageCollection};

struct Renderable<'a> {
	pub depth: i32,
	pub image: AssetHandle<Image>,
	pub texture: &'a Texture<'a>,
	pub rect: Rect,
}

pub struct RenderSystem<'a> {
	sdl: &'a SdlContext<'a>,
	renderables: Vec<Renderable<'a>>,
}

impl<'a> RenderSystem<'a> {
	pub fn new(sdl: &'a SdlContext) -> Self {
		RenderSystem {
			sdl: sdl,
			renderables: Vec::default(),
		}
	}
}

impl<'a, 's> System<'s> for RenderSystem<'a> {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Sprite>,
		FetchMut<'s, UpdatedSprites>,
		Fetch<'s, ImageCollection>
	);

	fn run(&mut self, (transforms, sprites, mut updated_sprites, image_collection): Self::SystemData) {
		if updated_sprites.entities.len() > 0 {
			for entity in &updated_sprites.entities {
				if let Some(sprite) = sprites.get(*entity) {
					let renderable = &mut self.renderables[sprite.renderable_index];
					renderable.depth = sprite.depth;

					if renderable.image != sprite.image && sprite.image.is_valid() {
						renderable.image = sprite.image;
						let image = image_collection.get(sprite.image);
						renderable.texture = self.sdl.textures.at(image.texture_index);
					}
				}
			}

			self.renderables.sort_by(|a, b| a.depth.cmp(&b.depth));
			updated_sprites.entities.clear();
		}

		for (transform, sprite) in (&transforms, &sprites).join() {
			let renderable = &mut self.renderables[sprite.renderable_index];
			renderable.rect.x = transform.position.x as i32;
			renderable.rect.y = transform.position.y as i32;
		}

		let canvas = &mut self.sdl.canvas.borrow_mut();
		for r in &self.renderables {
			canvas.copy(&r.texture, None, r.rect).unwrap();
		}
	}
}
