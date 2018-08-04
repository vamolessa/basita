use sdl2::rect::Point;

use specs::{Join, ReadStorage, System, Write};

use super::components::Sprite;
use super::resources::{ImageInstance, RenderLayers};
use core::components::Transform;

pub struct RenderSystem;

impl<'s> System<'s> for RenderSystem {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Sprite>,
		Write<'s, RenderLayers>,
	);

	fn run(&mut self, (transforms, sprites, mut layers): Self::SystemData) {
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
				position: Point::new(transform.position.x as i32, transform.position.y as i32),
			});
		}
	}
}
