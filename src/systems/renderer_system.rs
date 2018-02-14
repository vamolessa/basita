use application::Application;
use components::sprite::Sprite;
use components::component::ComponentCollection;
use sdl2::rect::Rect;

pub struct RendererSystem<'a> {
	pub sprite_collection: &'a ComponentCollection<Sprite<'a>>,
}

impl<'a> RendererSystem<'a> {
	pub fn update(&self, app: &mut Application) {
		for sprite in &self.sprite_collection.all {
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
