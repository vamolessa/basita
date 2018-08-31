use sdl2::pixels::Color;
use sdl2::rect::Point;

pub enum GuiVariant {
	Text(String),
}

pub struct GuiCommand {
	pub position: Point,
	pub color: Color,
	pub variant: GuiVariant,
}
