use sdl2::rect::Point;

use core::assets::{Asset, AssetLoadError, AssetLoader};
use sdl::{SdlLoader, SdlStorage};

pub struct Image {
	pub texture_index: usize,
	pub center: Point,
}

impl Asset for Image {}

impl<'a> AssetLoader<'a, Image> for SdlLoader {
	type Storage = SdlStorage<'a>;

	fn load(&'a self, path: &str, storage: &mut Self::Storage) -> Result<Image, AssetLoadError> {
		self.load_texture(path, storage).map(|index| {
			let texture = storage.texture_storage.at(index);
			let query = texture.query();

			Image {
				texture_index: index,
				center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
			}
		})
	}
}
