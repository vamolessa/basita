use specs::{Component, VecStorage};

use super::assets::Image;
use core::assets::AssetHandle;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sprite {
	pub layer: usize,
	pub image: AssetHandle<Image>,
	pub flip_horizontal: bool,
	pub flip_vertical: bool,
}

impl Component for Sprite {
	type Storage = VecStorage<Self>;
}
