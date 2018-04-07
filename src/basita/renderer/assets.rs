use sdl2::render::Texture;
use sdl2::rect::Point;

use core::assets::{Asset, AssetLoadError, AssetLoader};
use sdl::Textures;

pub struct Image {
	pub texture_index: usize,
	pub center: Point,
}

impl Image {
	fn new<'a>(texture: &Texture<'a>, index: usize) -> Self {
		let query = texture.query();

		Image {
			texture_index: index,
			center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
		}
	}
}

impl Asset for Image {}

impl<'a> AssetLoader<'a, Image> for Textures<'a> {
	fn load(&'a mut self, path: &str) -> Result<Image, AssetLoadError> {
		self.load_texture(path).map(|(t, i)| Image::new(t, i))
	}
}