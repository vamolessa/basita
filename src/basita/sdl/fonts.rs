use fxhash::FxHashMap;
use sdl2::pixels::Color;
use sdl2::ttf::{self, Font, Sdl2TtfContext};

use super::{SdlAssetStorage, SdlStorage, TextureLoader};
use core::assets::AssetLoadError;

pub struct FontGlyph {
	pub texture_index: usize,
	pub width: u32,
	pub height: u32,
}

pub type FontStorage<'a, 'b> = SdlAssetStorage<Font<'a, 'b>>;

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
	) -> Result<(usize, FxHashMap<char, FontGlyph>), AssetLoadError> {
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
					let texture_query = texture.query();

					glyphs.insert(
						c,
						FontGlyph {
							texture_index: storage.texture_storage.add(texture),
							width: texture_query.width,
							height: texture_query.height,
						},
					);
				}

				Ok((storage.font_storage.add(font), glyphs))
			}
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}