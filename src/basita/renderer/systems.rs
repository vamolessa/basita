use sdl2::rect::Rect;
use sdl2::render::Texture;

use specs::{Fetch, FetchMut, Join, ReadStorage, System};

use super::assets::Image;
use super::components::Sprite;
use super::resources::{DirtySprites, ImageCollection};
use core::assets::AssetHandle;
use core::components::Transform;
use sdl::SdlContext;

struct Renderable<'a> {
	pub depth: i32,
	pub image: AssetHandle<Image>,
	pub rect: Rect,
	pub _phantom: ::std::marker::PhantomData<&'a ()>,
}

pub struct RenderSystem<'a> {
	sdl: &'a SdlContext<'a>,
	renderables: Vec<Renderable<'a>>,
}

impl<'a> RenderSystem<'a> {
	pub fn new(sdl: &'a SdlContext<'a>) -> Self {
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
			renderable.rect.x = transform.position.x as i32;
			renderable.rect.y = transform.position.y as i32;
		}

		let mut canvas = self.sdl.canvas.borrow_mut();
		let textures = self.sdl.textures.borrow();

		for r in &self.renderables {
			let image = image_collection.get(r.image);
			let texture = textures.at(image.texture_index);

			canvas.copy(texture, None, r.rect).unwrap();
		}
	}
}
