use sdl2::rect::Rect;

use specs::{Fetch, FetchMut, Join, ReadStorage, System};

use super::components::Sprite;
use super::resources::{DirtySprites, ImageInstances, Images};
use core::components::Transform;
use sdl::{SdlContext, SdlStorage};

pub struct RenderSystem<'a: 'b, 'b> {
	sdl_context: &'a SdlContext,
	sdl_storage: &'b SdlStorage<'a>,
}

impl<'a, 'b> RenderSystem<'a, 'b> {
	pub fn new(sdl_context: &'a SdlContext, sdl_storage: &'b SdlStorage<'a>) -> Self {
		RenderSystem {
			sdl_context: sdl_context,
			sdl_storage: sdl_storage,
		}
	}
}

impl<'a, 'b, 's> System<'s> for RenderSystem<'a, 'b> {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Sprite>,
		FetchMut<'s, DirtySprites>,
		Fetch<'s, Images>,
		FetchMut<'s, ImageInstances>,
	);

	fn run(
		&mut self,
		(transforms, sprites, mut dirty_sprites, images, mut image_instances): Self::SystemData,
	) {
		if dirty_sprites.entities.len() > 0 {
			for entity in &dirty_sprites.entities {
				if let Some(sprite) = sprites.get(*entity) {
					if sprite.image_instance_index >= image_instances.instances.len() {
						image_instances
							.instances
							.resize_default(sprite.image_instance_index + 1);
					}

					let renderable = &mut image_instances.instances[sprite.image_instance_index];
					renderable.depth = sprite.depth;
					renderable.image = sprite.image;
				} else {
					
				}
			}

			image_instances
				.instances
				.sort_by(|a, b| a.depth.cmp(&b.depth));
			dirty_sprites.entities.clear();
		}

		for (transform, sprite) in (&transforms, &sprites).join() {
			let renderable = &mut image_instances.instances[sprite.image_instance_index];
			renderable.position.x = transform.position.x as i32;
			renderable.position.y = transform.position.y as i32;
		}

		let mut canvas = self.sdl_context.canvas.borrow_mut();
		let textures = self.sdl_storage.texture_storage.borrow();

		for r in &image_instances.instances {
			let image = images.get(r.image);
			let texture = textures.at(image.texture_index);

			let position = r.position + image.center;
			let rect = Rect::from_center(position, image.width, image.height);

			canvas.copy(texture, None, rect).unwrap();
		}
	}
}
