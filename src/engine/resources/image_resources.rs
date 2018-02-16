use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};

use super::{ResourceLoader, ResourceManager};

pub struct ImageResource<'a> {
	pub texture: Texture<'a>,
}

pub type ImageResources<'a> = ResourceManager<'a, ImageResource<'a>, TextureCreator<WindowContext>>;

impl<'a, T> ResourceLoader<'a, ImageResource<'a>> for TextureCreator<T> {
	fn load(&'a self, path: &str) -> Result<ImageResource, String> {
		self.load_texture(path)
			.map(|texture| ImageResource { texture: texture })
	}
}
