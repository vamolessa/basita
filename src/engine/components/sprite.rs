use resources::Image;

use super::{Component, Transform};

pub struct Sprite<'a> {
	pub depth: i32,
	pub image: &'a Image<'a>,
	pub transform: &'a Transform,
}

impl<'a> Component for Sprite<'a> {}
