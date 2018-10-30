use sdl2::pixels::Color;
use sdl2::rect::Point;

use renderer::assets::Font;
use renderer::resources::{RenderCommand, RenderCommands, RenderVariant};

pub fn label(
	commands: &mut RenderCommands,
	position: Point,
	layer: usize,
	font: &Font,
	color: Color,
	text: &String,
) {
	let mut x_offset: i32 = 0;
	for c in text.chars() {
		if let Some(glyph) = font.glyphs.get(&c) {
			commands.push(RenderCommand {
				layer: layer,
				position: Point::new(position.x + x_offset, position.y),
				color: color,
				variant: RenderVariant::Texture(glyph.texture_index),
			});

			x_offset += glyph.width as i32;
		}
	}
}
