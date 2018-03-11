use specs::{Component, VecStorage};

use core::assets::AssetHandle;
use super::assets::Image;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sprite {
	pub depth: i32,
	//pub image_resource: AssetHandle<Image<'a>>,
}

impl Component for Sprite {
	type Storage = VecStorage<Self>;
}