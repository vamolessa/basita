use components::{Component, Transform};
use resources::Image;

pub struct Sprite<'a> {
	pub depth: i32,
	pub image: &'a Image<'a>,
	pub transform: &'a Transform,
}

impl<'a> Component for Sprite<'a> {}
