use sdl2::pixels::Color;

use crate::core::assets::AssetHandle;
use crate::math::Vector2;

use super::assets::Image;

#[derive(Default, Clone, Copy, Debug)]
pub struct Camera {
	pub position: Vector2,
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
	pub layer: usize,
	pub color: Color,
	pub image: AssetHandle<Image>,
	pub flip_horizontal: bool,
	pub flip_vertical: bool,
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
