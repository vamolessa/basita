use specs::{Component, VecStorage};

use super::assets::Image;
use core::assets::AssetHandle;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sprite {
	pub layer: usize,
	pub image: AssetHandle<Image>,
}

impl Component for Sprite {
	type Storage = VecStorage<Self>;
}
