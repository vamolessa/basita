use sdl2::rect::Point;

use specs::Entity;

use super::assets::Image;
use core::assets::{AssetCollection, AssetHandle};

pub type Images = AssetCollection<Image>;

#[derive(Default)]
pub struct DirtySprites {
	pub entities: Vec<Entity>,
}

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

#[derive(Default)]
pub struct ImageInstances {
	pub instances: Vec<ImageInstance>,
}
