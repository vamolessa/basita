use application::Application;
use components::sprite::Sprites;
use sdl2::rect::Rect;

pub struct RendererSystem<'a> {
	pub sprites: &'a Sprites<'a>,
}

impl<'a> RendererSystem<'a> {
	pub fn update(&self, app: &mut Application) {
		for sprite in &self.sprites.all {
			let query = sprite.image.texture.query();
			app.canvas
				.copy(
					&sprite.image.texture,
					None,
					Rect::from((sprite.x, sprite.y, query.width, query.height)),
				)
				.unwrap();
		}
	}
}
