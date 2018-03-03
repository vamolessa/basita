use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::Point;
use sdl2::video::WindowContext;

use super::{ResourceHandle, ResourceLoader, ResourceManager};

pub struct ImageResource<'a> {
	pub texture: Texture<'a>,
	pub center: Point,
}

impl<'a> ImageResource<'a> {
	fn new(texture: Texture<'a>) -> Self {
		let query = texture.query();

		ImageResource {
			texture: texture,
			center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
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
