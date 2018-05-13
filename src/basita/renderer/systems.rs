use sdl2::rect::{Rect, Point};

use specs::{Fetch, FetchMut, Join, ReadStorage, System};

use super::components::Sprite;
use super::resources::{Layers, Images, ImageInstance};
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
		Fetch<'s, Images>,
		FetchMut<'s, Layers>,
	);

	fn run(
		&mut self,
		(transforms, sprites, images, mut layers): Self::SystemData,
	) {
		for layer in layers.iter_mut() {
			layer.clear();
		}

		for (transform, sprite) in (&transforms, &sprites).join() {
			if sprite.layer_index >= layers.len() {
				layers.resize_default(sprite.layer_index + 1);
			}

			let mut layer = &mut layers[sprite.layer_index];
			layer.push(ImageInstance {
				image: sprite.image,
				position: Point::new(
					transform.position.x as i32,
					transform.position.y as i32
				)
			});
		}

		let mut canvas = self.sdl_context.canvas.borrow_mut();
		let textures = self.sdl_storage.texture_storage.borrow();

		for layer in layers.iter() {
			for image_instance in layer.iter() {
				let image = images.get(image_instance.image);
				let texture = textures.at(image.texture_index);

				let position = image_instance.position + image.center;
				let rect = Rect::from_center(position, image.width, image.height);

				canvas.copy(texture, None, rect).unwrap();
			}
		}
	}
}
