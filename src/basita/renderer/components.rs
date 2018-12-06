use sdl2::pixels::Color;
use specs::{Component, DenseVecStorage, VecStorage};

use crate::core::assets::AssetHandle;
use crate::math::Vector2;

use super::assets::Image;

#[derive(Default, Clone, Copy, Debug)]
pub struct Camera {
	pub position: Vector2,
}

impl Component for Camera {
	type Storage = DenseVecStorage<Self>;
}

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
