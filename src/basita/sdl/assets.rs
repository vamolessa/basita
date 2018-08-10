use sdl2::render::Texture;
use sdl2::ttf::Font;

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

#[derive(Default)]
pub struct FontStorage<'a, 'b> {
	textures: Vec<Font<'a, 'b>>,
}

impl<'a, 'b> FontStorage<'a, 'b> {
	pub fn add(&mut self, texture: Font<'a, 'b>) -> usize {
		let index = self.textures.len();
		self.textures.push(texture);
		index
	}

	pub fn at(&self, index: usize) -> &Font<'a, 'b> {
		&self.textures[index]
	}
}
