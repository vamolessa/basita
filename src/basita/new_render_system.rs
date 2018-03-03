use sdl2::rect::Rect;

use sdl::SdlContext;
use World;
use assets::{Image, AssetHandle};
use components::{Sprite, Transform};

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
			renderables: Vec::default()
		}
	}

	pub fn sort_by_depth(&mut self) {
		self.renderables.sort_by(|a, b| a.depth.cmp(&b.depth));
	}
}

impl<'a, W: World> System<W> for RenderSystem<'a> {
	fn update(&mut self, world: &mut W) {
		for _s in world.components::<Sprite>().iter() {
			let t = Transform::identity(); // t in world.components::<Transform>().iter()

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
	}
}

pub trait System<W: World> {
	fn init(&mut self, &mut W) {}
	fn update(&mut self, &mut W);
}