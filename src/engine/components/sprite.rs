use super::Component;

use resources::ImageResourceHandle;

pub struct Sprite<'a> {
	pub depth: i32,
	pub image_resource: ImageResourceHandle<'a>,
}

impl<'a> Component for Sprite<'a> {}
