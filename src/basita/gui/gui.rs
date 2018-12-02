use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use math::Vector2;
use renderer::assets::{Font, Image};
use renderer::resources::{RenderCommand, RenderCommands, RenderVariant};

pub struct Gui<'a> {
	pub render_commands: &'a mut RenderCommands,
	pub layer: usize,
	pub color: Color,
	pub font: &'a Font,
}

impl<'a> Gui<'a> {
	pub fn new(render_commands: &'a mut RenderCommands, font: &'a Font) -> Self {
		Gui {
			render_commands: render_commands,
			layer: 0,
			color: Color::RGBA(255, 255, 255, 255),
			font: font,
		}
	}

	pub fn label(&mut self, position: Point, text: &str, anchor: Vector2) -> Rect {
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
				self.render_commands.commands.push(RenderCommand {
					layer: self.layer,
					position: Point::new(position.x + x_offset, position.y),
					color: self.color,
					variant: RenderVariant::Texture(glyph.texture_index),
				});

				x_offset += glyph.width as i32;
			}
		}

		Rect::new(position.x, position.y, width, height)
	}

	pub fn image(&mut self, position: Point, image: &Image) {
		self.render_commands
			.add_texture(self.layer, self.color, position, image.index);
	}

	pub fn rect(&mut self, position: Point, width: u32, height: u32) {
		self.render_commands
			.add_rect(self.layer, self.color, position, width, height);
	}

	pub fn rect_fill(&mut self, position: Point, width: u32, height: u32) {
		self.render_commands
			.add_rect_fill(self.layer, self.color, position, width, height);
	}

	pub fn line(&mut self, position: Point, to_position: Point) {
		self.render_commands
			.add_line(self.layer, self.color, position, to_position);
	}

	pub fn point(&mut self, position: Point) {
		self.render_commands
			.add_point(self.layer, self.color, position);
	}
}
