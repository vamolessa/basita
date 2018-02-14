use image_resources::Image;
use components::component::Component;

pub struct Sprite<'a> {
	pub x: i32,
	pub y: i32,
	pub image: &'a Image<'a>,
}

impl<'a> Component for Sprite<'a> {}
