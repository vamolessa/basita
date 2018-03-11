use sdl2::rect::Rect;

use specs::{System,ReadStorage};

use sdl::SdlContext;
use assets::{AssetHandle, Image};
use core::components::Transform;
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
		/*
		let sprites = world.components::<Sprite>();
		let transforms = world.components::<Transform>();

		for (s, t) in (sprites, transforms).iter(world.entities().len()) {
			let renderable_index = 0; //s.renderable_index;
			let renderable = &mut self.renderables[renderable_index];
			renderable.rect.x = t.position.x as i32;
			renderable.rect.y = t.position.y as i32;
		}

		let images = world.assets::<Image<'a>>();
		let canvas = &mut self.sdl.canvas.borrow_mut();

		for r in &self.renderables {
			let image = &images.get(r.image_resource);
			canvas.copy(&image.texture, None, r.rect).unwrap();
		}
		*/
	}
}
