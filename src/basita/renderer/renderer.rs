use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use resources::{ImageResourceHandle, ImageResources};

pub struct Renderable<'a> {
	pub depth: i32,
	pub image_resource: ImageResourceHandle<'a>,
	pub rect: Rect,
}

pub struct Renderer<'a> {
	renderables: Vec<Renderable<'a>>,
}

impl<'a> Renderer<'a> {
	pub fn new() -> Self {
		Renderer {
			renderables: Vec::default(),
		}
	}

	pub fn add(&mut self, renderable: Renderable<'a>) -> usize {
		self.renderables.push(renderable);
		self.sort_by_depth();

		self.renderables.len() - 1
	}

	pub fn get(&self, index: usize) -> &Renderable<'a> {
		&self.renderables[index]
	}

	pub fn get_mut(&mut self, index: usize) -> &mut Renderable<'a> {
		&mut self.renderables[index]
	}

	pub fn remove(&mut self, index: usize) {
		self.renderables.remove(index);
	}

	pub fn remove_all(&mut self) {
		self.renderables.clear()
	}

	pub fn sort_by_depth(&mut self) {
		self.renderables.sort_by(|a, b| a.depth.cmp(&b.depth));
	}

	pub fn render(&self, canvas: &mut Canvas<Window>, images: &ImageResources<'a>) {
		for r in &self.renderables {
			let image = &images.get(r.image_resource);
			canvas.copy(&image.texture, None, r.rect).unwrap();
		}
	}
}
