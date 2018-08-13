use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::{SdlContext, SdlStorage};
use core::assets::AssetLoadError;

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

#[derive(Default)]
pub struct TextureStorage<'a> {
	textures: Vec<Texture<'a>>,
}

impl<'a> TextureStorage<'a> {
	pub fn add(&mut self, texture: Texture<'a>) -> usize {
		let index = self.textures.len();
		self.textures.push(texture);
		index
	}

	pub fn at(&self, index: usize) -> &Texture<'a> {
		&self.textures[index]
	}
}
