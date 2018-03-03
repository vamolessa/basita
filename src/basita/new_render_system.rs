use World;
use components::{Sprite, Transform};
use renderer::Renderer;

pub struct RenderSystem;

impl RenderSystem {
	pub fn update<W: World>(&self, world: &mut W) {
		let mut renderer = Renderer::new(); //world.renderer_mut()

		for _s in world.components::<Sprite>().iter() {
			let t = Transform::identity(); // t in world.components::<Transform>().iter()

			let renderable_index = 0; //s.renderable_index;
			let renderable = renderer.get_mut(renderable_index);
			renderable.rect.x = t.position.x as i32;
			renderable.rect.y = t.position.y as i32;
		}

		//renderer.render(canvas, images)
	}
}
