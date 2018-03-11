use sdl2::rect::Rect;
use sdl2::render::Texture;

use specs::{Join, ReadStorage, System};

use sdl::SdlContext;
use core::components::Transform;
use super::components::Sprite;

struct Renderable<'a> {
	pub depth: i32,
	pub texture: &'a Texture<'a>,
	pub rect: Rect,
}

pub struct RenderSystem<'a> {
	sdl: &'a SdlContext<'a>,
	renderables: Vec<Renderable<'a>>,
}

impl<'a> RenderSystem<'a> {
	pub fn new(sdl: &'a SdlContext) -> Self {
		RenderSystem {
			sdl: sdl,
			renderables: Vec::default(),
		}
	}
}

impl<'a> System<'a> for RenderSystem<'a> {
	type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Sprite>);

	fn run(&mut self, (transforms, sprites): Self::SystemData) {
		for (transform, sprite) in (&transforms, &sprites).join() {
			let renderable = &mut self.renderables[sprite.renderable_index];
			renderable.rect.x = transform.position.x as i32;
			renderable.rect.y = transform.position.y as i32;
		}

		self.renderables.sort_by(|a, b| a.depth.cmp(&b.depth));

		let canvas = &mut self.sdl.canvas.borrow_mut();
		for r in &self.renderables {
			canvas.copy(&r.texture, None, r.rect).unwrap();
		}
	}
}
