use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use sdl_context::SdlContext;

use super::super::{Engine, EngineResources};

#[derive(Copy, Clone)]
pub struct ImageResource {
	index: usize,
}

pub struct ImageResources<'a> {
	textures: Vec<Texture<'a>>,
}

impl<'a> ImageResources<'a> {
	pub fn new() -> ImageResources<'a> {
		ImageResources {
			textures: Vec::new(),
		}
	}

	pub fn load(&mut self, _sdl: &'a SdlContext, _image_path: &Path) -> ImageResource {
		//let texture = sdl.texture_creator.load_texture(image_path).unwrap();
		//self.textures.push(texture);

		ImageResource {
			index: self.textures.len() - 1,
		}
	}

	pub fn get_texture(&self, image: ImageResource) -> &Texture {
		&self.textures[image.index]
	}
}

pub fn load<'a, 'b>(
	engine: &'a Engine,
	engine_resources: &mut EngineResources<'a>,
	image_path: &Path,
) -> ImageResource {
	let texture = engine
		.sdl_context
		.texture_creator
		.load_texture(image_path)
		.unwrap();

	engine_resources.image_resources.textures.push(texture);

	ImageResource {
		index: engine_resources.image_resources.textures.len() - 1,
	}
}
