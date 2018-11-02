use sdl2::rect::Point;

use super::gui::Gui;
use renderer::resources::{RenderCommand, RenderVariant};

impl<'a> Gui<'a> {
	pub fn label(&mut self, position: Point, text: &str) {
		let mut x_offset: i32 = 0;
		for c in text.chars() {
			if let Some(glyph) = self.font.glyphs.get(&c) {
				self.render_commands.push(RenderCommand {
					layer: self.layer,
					position: Point::new(position.x + x_offset, position.y),
					color: self.color,
					variant: RenderVariant::Texture(glyph.texture_index),
				});

				x_offset += glyph.width as i32;
			}
		}
	}
}
