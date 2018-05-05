use sdl2::rect::Point;

use specs::Entity;

use super::assets::Image;
use core::assets::{AssetCollection, AssetHandle};

pub type Images = AssetCollection<Image>;
pub type DirtySprites = Vec<Entity>;
pub type ImageInstances = Vec<ImageInstance>;

pub struct ImageInstance {
	pub depth: i32,
	pub image: AssetHandle<Image>,
	pub position: Point,
}

impl Default for ImageInstance {
	fn default() -> Self {
		ImageInstance {
			depth: 0,
			image: Default::default(),
			position: Point::new(0, 0),
		}
	}
}
