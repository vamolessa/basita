use sdl2::pixels::Color;
use specs::{Component, VecStorage};

use super::assets::{Font, Image};
use core::assets::AssetHandle;

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
	pub layer: usize,
	pub color: Color,
	pub image: AssetHandle<Image>,
	pub flip_horizontal: bool,
	pub flip_vertical: bool,
}

impl Component for Sprite {
	type Storage = VecStorage<Self>;
}

impl Default for Sprite {
	fn default() -> Self {
		Sprite {
			layer: 0,
			color: Color::RGB(255, 255, 255),
			image: AssetHandle::default(),
			flip_horizontal: false,
			flip_vertical: false,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Text {
	pub layer: usize,
	pub color: Color,
	pub font: AssetHandle<Font>,
	pub text: String,
}

impl Component for Text {
	type Storage = VecStorage<Self>;
}

impl Default for Text {
	fn default() -> Self {
		Text {
			layer: 0,
			color: Color::RGB(255, 255, 255),
			font: AssetHandle::default(),
			text: String::new(),
		}
	}
}
