use sdl2::rect::Point;

use core::assets::{Asset, AssetLoadError, AssetLoader};
use sdl::{TextureLoader, TextureStorage, Textures};

pub struct Image {
	pub texture_index: usize,
	pub center: Point,
}

impl Asset for Image {}

impl<'a> AssetLoader<'a, Image> for Textures<'a> {
	fn load(&'a mut self, path: &str) -> Result<Image, AssetLoadError> {
		self.load_texture(path).map(|(t, i)| {
			let query = t.query();

			Image {
				texture_index: i,
				center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
			}
		})
	}
}

impl<'a> AssetLoader<'a, Image> for (TextureLoader, TextureStorage<'a>) {
	fn load(&'a mut self, path: &str) -> Result<Image, AssetLoadError> {
		self.0.load_texture(path, &mut self.1).map(|index| {
			let texture = self.1.at(index);
			let query = texture.query();

			Image {
				texture_index: index,
				center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
			}
		})
	}
}

impl<'a> AssetLoader<'a, Image> for (&'a TextureLoader, &'a mut TextureStorage<'a>) {
	fn load(&'a mut self, path: &str) -> Result<Image, AssetLoadError> {
		self.0.load_texture(path, &mut self.1).map(|index| {
			let texture = self.1.at(index);
			let query = texture.query();

			Image {
				texture_index: index,
				center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
			}
		})
	}
}
