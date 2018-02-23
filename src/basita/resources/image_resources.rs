use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::super::math::Vector2;

use super::{ResourceHandle, ResourceLoader, ResourceManager};

pub struct ImageResource<'a> {
	pub texture: Texture<'a>,
	pub center: Vector2,
}

impl<'a> ImageResource<'a> {
	fn new(texture: Texture<'a>) -> Self {
		let query = texture.query();

		ImageResource {
			texture: texture,
			center: Vector2::new(query.width as f32 * 0.5, query.height as f32 * 0.5),
		}
	}
}

pub type ImageResourceHandle<'a> = ResourceHandle<ImageResource<'a>>;
pub type ImageResources<'a> = ResourceManager<'a, ImageResource<'a>, TextureCreator<WindowContext>>;

impl<'a> ResourceLoader<'a, ImageResource<'a>> for TextureCreator<WindowContext> {
	fn load(&'a self, path: &str) -> Result<ImageResource, String> {
		self.load_texture(path).map(ImageResource::new)
	}
}
