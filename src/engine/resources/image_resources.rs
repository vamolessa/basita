use std::path::Path;

use sdl2;
use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
use sdl2::render::{Texture, TextureCreator};

use sdl_context::SdlContext;

pub struct Image<'a> {
	pub texture: Texture<'a>,
}

pub struct ImageResources {
	texture_creator: TextureCreator<sdl2::video::WindowContext>,
}

impl ImageResources {
pub fn new(sdl: &SdlContext) -> ImageResources {
		let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
		let texture_creator = sdl.canvas.texture_creator();

		ImageResources {
			texture_creator: texture_creator,
		}
	}

	pub fn load<'a>(&'a mut self, image_path: &Path) -> Image<'a> {
		Image {
			texture: self.texture_creator.load_texture(image_path).unwrap(),
		}
	}
}
