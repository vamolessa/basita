use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::assets::{Font, Image};
use core::assets::{AssetCollection, AssetLoadRequests};

pub type Images = AssetCollection<Image>;
pub type ImageLoadRequests = AssetLoadRequests<Image>;

pub type Fonts = AssetCollection<Font>;
pub type FontLoadRequests = AssetLoadRequests<Font>;

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
	pub color: Color,
	pub position: Point,
	pub variant: RenderVariant,
}

#[derive(Default)]
pub struct RenderCommands {
	pub commands: Vec<RenderCommand>,
}

impl RenderCommands {
	pub fn add_texture(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		texture_index: usize,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Texture(texture_index),
		})
	}

	pub fn add_texture_ex(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		texture_index: usize,
		flip_horizontally: bool,
		flip_vertically: bool,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::TextureEx(texture_index, flip_horizontally, flip_vertically),
		})
	}

	pub fn add_rect(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		width: u32,
		height: u32,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Rect(width, height),
		})
	}

	pub fn add_rect_fill(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		width: u32,
		height: u32,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::RectFill(width, height),
		})
	}

	pub fn add_line(&mut self, layer: usize, color: Color, position: Point, to: Point) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Line(to),
		})
	}

	pub fn add_point(&mut self, layer: usize, color: Color, position: Point) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Point,
		})
	}
}
