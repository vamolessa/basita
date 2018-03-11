use sdl2::rect::Rect;

use specs::{System, ReadStorage, Join};

use sdl::SdlContext;
use core::assets::{AssetHandle};
use core::components::Transform;
use super::assets::Image;
use super::components::Sprite;

struct Renderable<'a> {
	pub depth: i32,
	pub image_resource: AssetHandle<Image<'a>>,
	pub rect: Rect,
}

pub struct RenderSystem<'a> {
	sdl: &'a SdlContext,
	renderables: Vec<Renderable<'a>>,
}

impl<'a> RenderSystem<'a> {
	pub fn new(sdl: &'a SdlContext) -> Self {
		RenderSystem {
			sdl: sdl,
			renderables: Vec::default(),
		}
	}

	pub fn sort_by_depth(&mut self) {
		self.renderables.sort_by(|a, b| a.depth.cmp(&b.depth));
	}
}

impl<'a> System<'a> for RenderSystem<'a> {
	type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Sprite>);

	fn run(&mut self, (transforms, sprites): Self::SystemData) {
		for (transform, sprite) in (&transforms, &sprites).join() {
			let renderable_index = 0; //sprite.renderable_index;
			let renderable = &mut self.renderables[renderable_index];
			renderable.rect.x = transform.position.x as i32;
			renderable.rect.y = transform.position.y as i32;
		}

	/*
		let images = world.assets::<Image<'a>>();
		let canvas = &mut self.sdl.canvas.borrow_mut();

		for r in &self.renderables {
			let image = &images.get(r.image_resource);
			canvas.copy(&image.texture, None, r.rect).unwrap();
		}
	*/
	}
}
