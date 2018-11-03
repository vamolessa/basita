use sdl2::rect::Point;

use math::Vector2;

use super::gui::Gui;
use renderer::resources::{RenderCommand, RenderVariant};

impl<'a> Gui<'a> {
	pub fn label(&mut self, position: Point, text: &str, anchor: Vector2) {
		let mut width: u32 = 0;
		let mut height: u32 = 0;
		let mut x_offset: i32 = 0;

		for c in text.chars() {
			if let Some(glyph) = self.font.glyphs.get(&c) {
				width += glyph.width;
				height = height.max(glyph.height);
			}
		}

		let mut position = position;
		position.x -= (width as f32 * anchor.x) as i32;
		position.y -= (height as f32 * anchor.y) as i32;

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
