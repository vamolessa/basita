use sdl2::pixels::Color;

use renderer::assets::Font;
use renderer::resources::RenderCommands;

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
}
