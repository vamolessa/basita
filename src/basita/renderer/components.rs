use specs::{Component, VecStorage};

use core::assets::AssetHandle;
use super::assets::Image;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sprite {
	pub layer_index: usize,
	pub image: AssetHandle<Image>,
}

impl Component for Sprite {
	type Storage = VecStorage<Self>;
}