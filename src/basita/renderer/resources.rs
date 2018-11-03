use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::assets::{Font, Image};
use core::assets::{AssetCollection, AssetLoadRequests};

pub type Images = AssetCollection<Image>;
pub type ImageLoadRequests = AssetLoadRequests<Image>;

pub type Fonts = AssetCollection<Font>;
pub type FontLoadRequests = AssetLoadRequests<Font>;

pub type RenderCommands = Vec<RenderCommand>;

pub enum RenderVariant {
	Texture(usize),
	TextureEx(usize, bool, bool),
	Rect(u32, u32),
	RectFill(u32, u32),
	Line(Point),
	Point,
}

pub struct RenderCommand {
	pub layer: usize,
	pub position: Point,
	pub color: Color,
	pub variant: RenderVariant,
}
