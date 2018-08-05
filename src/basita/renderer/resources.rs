use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::assets::Image;
use core::assets::AssetCollection;

pub type Images = AssetCollection<Image>;
pub type RenderCommands = Vec<RenderCommand>;

pub enum RenderVariant {
	Texture(usize),
	Rect(Color, u32, u32),
}

pub struct RenderCommand {
	pub layer: usize,
	pub position: Point,
	pub render_variant: RenderVariant,
}
