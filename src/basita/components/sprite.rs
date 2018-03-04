use super::Component;

use assets::{AssetHandle, Image};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sprite<'a> {
	pub depth: i32,
	pub image_resource: AssetHandle<Image<'a>>,
}

impl<'a> Component for Sprite<'a> {}
