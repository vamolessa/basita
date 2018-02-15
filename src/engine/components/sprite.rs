use resources::ImageResource;

use super::Component;

pub struct Sprite {
	pub depth: i32,
	pub image: ImageResource,
}

impl Component for Sprite {}
