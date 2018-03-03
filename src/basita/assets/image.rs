use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::Point;
use sdl2::video::WindowContext;

use super::{Asset, AssetLoader, AssetLoadError};

pub struct Image<'a> {
	pub texture: Texture<'a>,
	pub center: Point,
}

impl<'a> Image<'a> {
	fn new(texture: Texture<'a>) -> Self {
		let query = texture.query();

		Image {
			texture: texture,
			center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
		}
	}
}

impl<'a> Asset for Image<'a> {}

impl<'a> AssetLoader<'a, Image<'a>> for TextureCreator<WindowContext> {
	fn load(&'a self, path: &str) -> Result<Image, AssetLoadError> {
		match self.load_texture(path) {
			Ok(texture) => Ok(Image::new(texture)),
			Err(message) => Err(AssetLoadError::new(message))
		}
	}
}
