use super::{Component, ComponentHandle, Transform};

use assets::{AssetHandle, Image};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sprite<'a> {
	pub depth: i32,
	pub image_resource: AssetHandle<Image<'a>>,

	pub transform: ComponentHandle<Transform>,
}

impl<'a> Component for Sprite<'a> {}
