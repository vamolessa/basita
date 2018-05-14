use sdl2::rect::Point;

use super::assets::Image;
use core::assets::{AssetCollection, AssetHandle};

pub type Images = AssetCollection<Image>;
pub type Layers = Vec<Vec<ImageInstance>>;

pub struct ImageInstance {
	pub image: AssetHandle<Image>,
	pub position: Point,
}

impl Default for ImageInstance {
	fn default() -> Self {
		ImageInstance {
			image: Default::default(),
			position: Point::new(0, 0),
		}
	}
}
