use fxhash::FxHashMap;
use sdl2::pixels::Color;
use sdl2::ttf::{self, Font, Sdl2TtfContext};

use super::{SdlStorage, TextureLoader};
use core::assets::AssetLoadError;

pub struct FontLoader {
	pub context: Sdl2TtfContext,
}

impl FontLoader {
	pub fn new() -> Self {
		FontLoader {
			context: ttf::init().unwrap(),
		}
	}

	pub fn load<'a>(
		&'a self,
		path: &str,
		size: u16,
		loader: &'a TextureLoader,
		storage: &mut SdlStorage<'a>,
	) -> Result<(usize, FxHashMap<char, usize>), AssetLoadError> {
		match self.context.load_font(path, size) {
			Ok(font) => {
				let alphabet = String::from(
					"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 _-:,'\".!?/\\",
				);
				let mut glyphs = FxHashMap::default();

				for c in alphabet.chars() {
					let surface = font
						.render_char(c)
						.blended(Color::RGBA(255, 255, 255, 255))
						.unwrap();
					let texture = loader
						.texture_creator
						.create_texture_from_surface(&surface)
						.unwrap();

					let glyph_index = storage.texture_storage.add(texture);
					glyphs.insert(c, glyph_index);
				}

				Ok((storage.font_storage.add(font), glyphs))
			}
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}

#[derive(Default)]
pub struct FontStorage<'a, 'b> {
	fonts: Vec<Font<'a, 'b>>,
}

impl<'a, 'b> FontStorage<'a, 'b> {
	pub fn add(&mut self, font: Font<'a, 'b>) -> usize {
		let index = self.fonts.len();
		self.fonts.push(font);
		index
	}

	pub fn at(&self, index: usize) -> &Font<'a, 'b> {
		&self.fonts[index]
	}
}
