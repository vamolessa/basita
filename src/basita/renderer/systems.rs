use sdl2::rect::Point;

use specs::{Join, Read, ReadStorage, System, Write};

use super::components::{Sprite, Text};
use super::resources::{Fonts, Images, RenderCommand, RenderCommands, RenderVariant};
use core::components::Transform;

pub struct RenderSystem;

impl<'s> System<'s> for RenderSystem {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Sprite>,
		ReadStorage<'s, Text>,
		Read<'s, Images>,
		Read<'s, Fonts>,
		Write<'s, RenderCommands>,
	);

	fn run(&mut self, (transforms, sprites, texts, images, fonts, mut commands): Self::SystemData) {
		commands.clear();

		for (transform, sprite) in (&transforms, &sprites).join() {
			let image = images.get(sprite.image);

			commands.push(RenderCommand {
				layer: sprite.layer,
				position: Point::new(transform.position.x as i32, transform.position.y as i32)
					- image.center,
				color: sprite.color,
				variant: RenderVariant::TextureEx(
					image.index,
					sprite.flip_horizontal,
					sprite.flip_vertical,
				),
			});
		}

		for (transform, text) in (&transforms, &texts).join() {
			let font = fonts.get(text.font);

			let mut x_offset: u32 = 0;
			for c in text.text.chars() {
				if let Some(glyph) = font.glyphs.get(&c) {
					commands.push(RenderCommand {
						layer: text.layer,
						position: Point::new(
							transform.position.x as i32 + x_offset as i32,
							transform.position.y as i32,
						),
						color: text.color,
						variant: RenderVariant::Texture(glyph.texture_index),
					});

					x_offset += glyph.width;
				}
			}
		}
	}
}
