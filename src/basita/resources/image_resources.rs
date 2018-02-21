use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use SdlContext;
use super::{ResourceHandle, ResourceLoader, ResourceManager};
use super::super::math::Vector2;

pub struct ImageResource<'a> {
	pub texture: Texture<'a>,
	pub center: Vector2,
}

pub type ImageResourceHandle<'a> = ResourceHandle<ImageResource<'a>>;
pub type ImageResources<'a> = ResourceManager<'a, ImageResource<'a>, SdlContext>;

impl<'a> ResourceLoader<'a, ImageResource<'a>> for SdlContext {
	fn load(&'a self, path: &str) -> Result<ImageResource, String> {
		self.texture_creator
			.load_texture(path)
			.map(|texture| ImageResource {
				texture: texture,
				center: Vector2::new(0.5, 0.5),
			})
	}
}
