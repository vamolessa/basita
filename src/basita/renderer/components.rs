use specs::{Component, VecStorage};

use assets::{AssetHandle, Image};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sprite {
	pub depth: i32,
	//pub image_resource: AssetHandle<Image<'a>>,
}

impl Component for Sprite {
	type Storage = VecStorage<Self>;
}