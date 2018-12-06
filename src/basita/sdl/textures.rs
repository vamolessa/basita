use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::{SdlAssetStorage, SdlContext, SdlStorage};
use crate::core::assets::AssetLoadError;

pub type TextureStorage<'a> = SdlAssetStorage<Texture<'a>>;

pub struct TextureLoader {
	pub texture_creator: TextureCreator<WindowContext>,
}

impl TextureLoader {
	pub fn new(sdl_context: &SdlContext) -> Self {
		TextureLoader {
			texture_creator: sdl_context.canvas.texture_creator(),
		}
	}

	pub fn load<'a>(
		&'a self,
		path: &str,
		storage: &mut SdlStorage<'a>,
	) -> Result<usize, AssetLoadError> {
		match self.texture_creator.load_texture(path) {
			Ok(texture) => Ok(storage.texture_storage.add(texture)),
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}
