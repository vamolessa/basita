use image_resources::Image;
use components::component::Component;
use std::cmp::Ordering;

pub struct Sprite<'a> {
	pub x: i32,
	pub y: i32,
	pub z: i32,
	pub image: &'a Image<'a>,
}

impl<'a> Component for Sprite<'a> {}

impl<'a> PartialEq for Sprite<'a> {
	fn eq(&self, other: &Self) -> bool {
		return self.z == other.z;
	}
}

impl<'a> Eq for Sprite<'a> {}

impl<'a> PartialOrd for Sprite<'a> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(Ordering::Equal)
	}
}

impl<'a> Ord for Sprite<'a> {
	fn cmp(&self, other: &Self) -> Ordering {
		Ordering::Equal
	}
}
