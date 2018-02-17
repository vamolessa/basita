use super::{Component, ComponentHandle, Transform};

use resources::ImageResourceHandle;

pub struct Sprite<'a> {
	pub depth: i32,
	pub image_resource: ImageResourceHandle<'a>,
	
	pub transform: ComponentHandle<Transform>,
}

impl<'a> Component for Sprite<'a> {}
