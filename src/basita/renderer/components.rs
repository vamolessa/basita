use sdl2::pixels::Color;
use specs::{Component, VecStorage};

use super::assets::{Font, Image};
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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Text {
	pub layer: usize,
	pub font: AssetHandle<Font>,
	pub text: String,
}

impl Component for Text {
	type Storage = VecStorage<Self>;
}
