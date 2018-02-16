use super::Component;

use resources::ResourceHandle;

pub struct Sprite {
	pub depth: i32,
	pub image_resource: ResourceHandle,
}

impl Component for Sprite {}
