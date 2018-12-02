use fxhash::FxHashMap;
use sdl2::rect::Point;

use core::assets::{Asset, AssetLoadError, AssetLoader};
use sdl::{FontGlyph, SdlLoader, SdlStorage};

pub struct Image {
	pub texture_index: usize,
	pub center: Point,
}

impl Asset for Image {
	type Id = String;
}

impl<'a> AssetLoader<'a, Image> for SdlLoader {
	type Storage = SdlStorage<'a>;

	fn load(
		&'a self,
		path: &<Image as Asset>::Id,
		storage: &mut Self::Storage,
	) -> Result<Image, AssetLoadError> {
		self.texture_loader.load(path, storage).map(|texture_index| {
			let texture = storage.texture_storage.at(texture_index);
			let query = texture.query();

			Image {
				texture_index: texture_index,
				center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
			}
		})
	}
}

pub struct Font {
	pub font_index: usize,
	pub size: u16,
	pub glyphs: FxHashMap<char, FontGlyph>,
}

impl Asset for Font {
	type Id = (String, u16);
}

impl<'a> AssetLoader<'a, Font> for SdlLoader {
	type Storage = SdlStorage<'a>;

	fn load(
		&'a self,
		&(ref path, size): &<Font as Asset>::Id,
		storage: &mut Self::Storage,
	) -> Result<Font, AssetLoadError> {
		self.font_loader
			.load(path, size, &self.texture_loader, storage)
			.map(|(font_index, glyphs)| Font {
				font_index: font_index,
				size: size,
				glyphs: glyphs,
			})
	}
}
