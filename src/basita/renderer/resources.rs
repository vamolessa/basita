use sdl2::rect::Point;

use super::assets::Image;
use core::assets::{AssetCollection, AssetHandle};

pub type Images = AssetCollection<Image>;
pub type RenderCommands = Vec<RenderCommand>;

pub struct RenderCommand {
	pub layer: usize,
	pub position: Point,
	pub texture_index: usize,
}
