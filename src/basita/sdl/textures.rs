use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use core::assets::AssetLoadError;

pub struct Textures<'a> {
	textures: Vec<Texture<'a>>,
	texture_creator: TextureCreator<WindowContext>,
}

impl<'a> Textures<'a> {
	pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
		Textures {
			textures: Vec::new(),
			texture_creator: texture_creator,
		}
	}

	pub fn load_texture(&'a mut self, path: &str) -> Result<(&Texture<'a>, usize), AssetLoadError> {
		match self.texture_creator.load_texture(path) {
			Ok(texture) => {
				let index = self.textures.len();
				self.textures.push(texture);
				Ok((&self.textures[index], index))
			}
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}
